//! Independent external invariant checks. No source of engine role/vote state.
use glade_carrier_api::*;
use std::collections::BTreeMap;

pub fn leader(views: &[ElectionView], traffic: &[MessageView]) -> Result<NodeKey, String> {
    if !traffic.iter().any(|message| {
        matches!(
            message.kind,
            ProtocolKind::VoteRequest | ProtocolKind::VoteResponse
        )
    }) {
        return Err("leader requires recorded engine vote traffic".into());
    }
    let candidates: Vec<_> = views
        .iter()
        .filter(|view| view.state == NodeState::Running && view.role == Role::Leader)
        .collect();
    for candidate in candidates {
        if candidate.vote.is_none() {
            continue;
        }
        let agreeing = views
            .iter()
            .filter(|view| {
                view.state == NodeState::Running
                    && view.leader == Some(candidate.scope.node)
                    && view.vote == candidate.vote
            })
            .count();
        if agreeing >= 2 {
            return Ok(candidate.scope.node);
        }
    }
    Err("no actual leader observed by a fixed-data voter quorum".into())
}
pub fn draws(
    events: &[Event],
    scope: NodeScope,
    expected: SampleRange,
) -> Result<Vec<u64>, String> {
    let mut values = Vec::new();
    for event in events {
        if let Event::SourceDraw {
            scope: actual,
            range,
            value,
            ..
        } = event
            && *actual == scope
        {
            if *range != expected || *value < range.lower || *value >= range.upper_exclusive {
                return Err("own stream/range violated".into());
            }
            values.push(*value);
        }
    }
    if values.is_empty() {
        return Err("constructor/reset must use the supplied stream".into());
    }
    Ok(values)
}
#[derive(Default)]
pub struct History {
    committed: BTreeMap<NodeKey, Vec<LogMark>>,
    leaders: BTreeMap<ElectionEpoch, NodeKey>,
}
impl History {
    pub fn check(&mut self, views: &[ElectionView], traffic: &[MessageView]) -> Result<(), String> {
        for view in views {
            if let Some(before) = self.committed.get(&view.scope.node)
                && !view.committed.starts_with(before)
            {
                return Err("committed protocol prefix regressed".into());
            }
            self.committed
                .insert(view.scope.node, view.committed.clone());
        }
        if let Ok(node) = leader(views, traffic) {
            let epoch = views
                .iter()
                .find(|view| view.scope.node == node)
                .and_then(|view| view.epoch)
                .ok_or("missing carrier-mapped standard election epoch")?;
            if let Some(prior) = self.leaders.insert(epoch, node)
                && prior != node
            {
                return Err(
                    "incompatible quorum leaders for the same carrier election epoch".into(),
                );
            }
        }
        Ok(())
    }
}

pub fn required_draws(
    events: &[Event],
    scope: NodeScope,
    range: SampleRange,
    expected: &[u64],
) -> Result<(), String> {
    if draws(events, scope, range)? != expected {
        return Err("trace differs from externally supplied stream/access footprint".into());
    }
    Ok(())
}
pub fn eligibility(
    now: LogicalInstant,
    first_eligible: LogicalInstant,
    messages: &[MessageView],
) -> Result<(), String> {
    if now.domain != first_eligible.domain {
        return Err("eligibility domains differ".into());
    }
    if now.nanos < first_eligible.nanos
        && messages
            .iter()
            .any(|message| message.kind == ProtocolKind::VoteRequest)
    {
        return Err("election before reviewed pinned eligibility boundary".into());
    }
    Ok(())
}
pub fn independent_clocks(
    before: &[NodeInventory],
    after: &[NodeInventory],
    selected: usize,
) -> Result<(), String> {
    if before.len() != after.len() {
        return Err("node inventory count changed".into());
    }
    for (index, (before, after)) in before.iter().zip(after).enumerate() {
        if index != selected && (before.clock != after.clock || before.work != after.work) {
            return Err("unselected node time/work advanced".into());
        }
    }
    Ok(())
}
pub fn paused_work(before: &[WorkView], after: &[WorkView]) -> Result<(), String> {
    if before != after {
        return Err("paused background task progressed".into());
    }
    Ok(())
}
