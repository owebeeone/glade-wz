//! Private common scheduling utility; neither an engine nor an executor provider.
use glade_carrier_api::*;
const POINT_ACTION_LIMIT: usize = 4096;
const TIME_POINT_LIMIT: usize = 4096;
pub(super) fn time_points(mut from: LogicalInstant, to: LogicalInstant) -> Vec<LogicalInstant> {
    assert_eq!(from.domain, to.domain, "boundary time domain");
    assert!(from.nanos <= to.nanos, "monotonic boundary schedule");
    let mut points = Vec::new();
    while from.nanos < to.nanos {
        assert!(
            points.len() < TIME_POINT_LIMIT,
            "bounded B0-04 time-point schedule"
        );
        let aligned = (from.nanos / crate::fixture::QUANTUM_NS)
            .checked_add(1)
            .and_then(|quantum| quantum.checked_mul(crate::fixture::QUANTUM_NS))
            .expect("bounded fixture time arithmetic");
        from.nanos = aligned.min(to.nanos);
        points.push(from);
    }
    points
}
pub(super) fn drain_point(
    mut step: impl FnMut(Option<WorkId>) -> Result<Vec<WorkView>, DriverError>,
) -> Result<Vec<WorkId>, DriverError> {
    let mut selected = Vec::new();
    let mut snapshot = step(None)?;
    snapshot.sort_by_key(|work| work.id);
    loop {
        let Some(work) = snapshot
            .iter()
            .find(|work| work.state == WorkState::Runnable)
        else {
            return Ok(selected);
        };
        assert!(
            selected.len() < POINT_ACTION_LIMIT,
            "bounded B0-04 fixed-instant action drain"
        );
        let id = work.id;
        snapshot = step(Some(id))?;
        snapshot.sort_by_key(|work| work.id);
        selected.push(id);
    }
}
mod witness;
