#!/usr/bin/env python3
"""Actual SIGKILL: complete externally retained APP/CONFIG/ENTRY originals.

Private Rust Debug records include every owned contract field and exact byte.
The journal/kernel stay running: this proves no power-loss durability claim.
"""
import argparse
import os
from pathlib import Path
import selectors
import signal
import subprocess
import tempfile
import time

ORIGINAL = ("APP_COMMAND", "APP_RECEIPT", "APP_ENTRY", "CONFIG_INTENT", "CONFIG_RECEIPT", "CONFIG_ENTRY")
RECOVERED = ("RECOVERED_APP_COMMAND", "RECOVERED_APP_LOOKUP", "RECOVERED_APP_RETRY", "RECOVERED_APP_ENTRY", "RECOVERED_APP_REPLAY",
             "RECOVERED_CONFIG_INTENT", "RECOVERED_CONFIG_LOOKUP", "RECOVERED_CONFIG_RETRY", "RECOVERED_CONFIG_ENTRY", "RECOVERED_CONFIG_REPLAY")


def record(records, line):
    key, separator, value = line.rstrip("\n").partition(" ")
    if key in ORIGINAL + RECOVERED + ("CUT",):
        if not separator or not value or key in records:
            raise AssertionError(f"malformed/duplicate oracle record {key}")
        records[key] = value


def expected_joint(intent):
    # Independently specified pre-application result; no source worker receipt
    # exists at this cut, so it cannot invent its own post-recovery oracle.
    return ("ConfigReceipt { intent: " + intent + ", outcome: Accepted, index: 4, configuration: Configuration { "
            "voters: [1, 2, 4], learners: [], voters_outgoing: [1, 2, 3], learners_next: [], auto_leave: false, index: 4 } }")


def verify(original, recovered):
    for name in ORIGINAL:
        assert name in original, f"missing parent-held original {name}"
    for name in RECOVERED:
        assert name in recovered, f"missing recovered field {name}"
    pairs = {
        "RECOVERED_APP_COMMAND": "APP_COMMAND", "RECOVERED_APP_LOOKUP": "APP_RECEIPT", "RECOVERED_APP_RETRY": "APP_RECEIPT", "RECOVERED_APP_ENTRY": "APP_ENTRY",
        "RECOVERED_CONFIG_INTENT": "CONFIG_INTENT", "RECOVERED_CONFIG_LOOKUP": "CONFIG_RECEIPT", "RECOVERED_CONFIG_RETRY": "CONFIG_RECEIPT", "RECOVERED_CONFIG_ENTRY": "CONFIG_ENTRY",
    }
    for current, previous in pairs.items():
        assert recovered[current] == original[previous], f"complete original mismatch: {current}"
    assert recovered["RECOVERED_APP_REPLAY"] == "Application(" + original["APP_RECEIPT"] + ")", "typed original application replay changed"
    assert recovered["RECOVERED_CONFIG_REPLAY"] == "Configuration(" + original["CONFIG_RECEIPT"] + ")", "typed original configuration replay changed"


def command(worker):
    return [str(worker), "--ignored", "--exact", "q3_process_worker", "--nocapture"]


def one(worker, kind):
    with tempfile.TemporaryDirectory(prefix="glade-q3-sigkill-") as directory:
        environment = {"PATH": "/usr/bin:/bin", "Q3_ROOT": directory, "Q3_MODE": "start", "Q3_KIND": kind}
        process = subprocess.Popen(command(worker), stdin=subprocess.PIPE, stdout=subprocess.PIPE, stderr=subprocess.STDOUT,
                                   env=environment)
        original = {}
        lines = []
        selector = selectors.DefaultSelector()
        selector.register(process.stdout, selectors.EVENT_READ)
        deadline = time.monotonic() + 30
        try:
            pending = b""
            while "CUT" not in original:
                assert time.monotonic() < deadline, f"worker did not reach {kind}: {''.join(lines)}"
                events = selector.select(max(0, deadline - time.monotonic()))
                assert events, f"worker timeout at {kind}: {''.join(lines)}"
                chunk = os.read(process.stdout.fileno(), 65536)
                assert chunk, f"worker exited before cut {kind}: {''.join(lines)}"
                pending += chunk
                assert len(pending) < 16 * 1024 * 1024, "unbounded worker oracle line"
                while b"\n" in pending:
                    raw, pending = pending.split(b"\n", 1)
                    line = raw.decode("utf-8") + "\n"
                    lines.append(line)
                    record(original, line)
            assert original["CUT"] == kind
            if kind == "joint-before-apply":
                original["CONFIG_RECEIPT"] = expected_joint(original["CONFIG_INTENT"])
            assert all(name in original for name in ORIGINAL), f"incomplete originals {kind}"
            process.send_signal(signal.SIGKILL)
            process.wait(timeout=10)
            assert process.returncode == -signal.SIGKILL
        finally:
            selector.close()
            if process.poll() is None:
                process.kill()
                process.wait(timeout=10)
            process.stdin.close()
            process.stdout.close()
        recovered_process = subprocess.run(command(worker), input="", env={**environment, "Q3_MODE": "recover"},
                                           stdout=subprocess.PIPE, stderr=subprocess.STDOUT, text=True, timeout=30)
        assert recovered_process.returncode == 0, recovered_process.stdout
        recovered = {}
        for line in recovered_process.stdout.splitlines():
            record(recovered, line)
        verify(original, recovered)
        print(f"Q3 SIGKILL {kind}: PASS (complete APP, CONFIG, ENTRY lookup/retry/typed replay)")


def self_test():
    original = dict(zip(ORIGINAL, ["complete-command", "Receipt { home: 1, generation: 2 }", "StoredEntry { bytes: [1, 7, 9] }",
                                   "complete-intent", "ConfigReceipt { outcome: Accepted, voters: [1, 2, 4] }", "StoredEntry { bytes: [2, 7, 9] }"]))
    recovered = {
        "RECOVERED_APP_COMMAND": original["APP_COMMAND"], "RECOVERED_APP_LOOKUP": original["APP_RECEIPT"], "RECOVERED_APP_RETRY": original["APP_RECEIPT"],
        "RECOVERED_APP_ENTRY": original["APP_ENTRY"], "RECOVERED_APP_REPLAY": "Application(" + original["APP_RECEIPT"] + ")",
        "RECOVERED_CONFIG_INTENT": original["CONFIG_INTENT"], "RECOVERED_CONFIG_LOOKUP": original["CONFIG_RECEIPT"], "RECOVERED_CONFIG_RETRY": original["CONFIG_RECEIPT"],
        "RECOVERED_CONFIG_ENTRY": original["CONFIG_ENTRY"], "RECOVERED_CONFIG_REPLAY": "Configuration(" + original["CONFIG_RECEIPT"] + ")",
    }
    verify(original, recovered)
    for key in RECOVERED:
        bad = {**recovered, key: recovered[key] + " changed"}
        try:
            verify(original, bad)
        except AssertionError:
            pass
        else:
            raise AssertionError(f"oracle missed changed complete field {key}")
    for bad in [{}, {**original, "APP_ENTRY": "changed bytes"}, {**original, "CONFIG_RECEIPT": "changed outcome/home"}]:
        try:
            verify(bad, recovered)
        except AssertionError:
            pass
        else:
            raise AssertionError("fresh-to-fresh oracle accepted missing/changed parent original")
    print("Q3 oracle adversarial self-test: PASS (10 recovered mutations, missing/changed parent originals)")


if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("--worker", type=Path)
    parser.add_argument("--self-test", action="store_true")
    arguments = parser.parse_args()
    if arguments.self_test:
        self_test()
    else:
        assert arguments.worker and arguments.worker.is_file(), "pass the compiled q3_process_crash executable"
        for cut in ("ack", "joint-before-apply", "snapshot-before-apply"):
            one(arguments.worker.resolve(), cut)
