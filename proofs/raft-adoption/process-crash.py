#!/usr/bin/env python3
"""Separate Q2 tier: actual SIGKILL at acknowledged and durable-before-apply cuts."""
import argparse
import os
from pathlib import Path
import selectors
import signal
import subprocess
import tempfile
import time

parser = argparse.ArgumentParser()
parser.add_argument('--worker', required=True, type=Path, help='compiled process_crash integration-test executable')
args = parser.parse_args()
worker = str(args.worker.resolve())
for mode, marker in [('write-ack', 'Q2_ACK 3 23'), ('write-cut', 'Q2_COMMIT_BEFORE_APPLY')]:
    with tempfile.TemporaryDirectory(prefix='glade-q2-kill-') as directory:
        env = {'GLADE_Q2_ROOT': directory, 'GLADE_Q2_MODE': mode}
        child = subprocess.Popen([worker, '--exact', 'q2_process_worker', '--ignored', '--nocapture'],
                                 env=env, stdin=subprocess.PIPE, stdout=subprocess.PIPE, stderr=subprocess.STDOUT)
        selector = selectors.DefaultSelector()
        selector.register(child.stdout, selectors.EVENT_READ)
        output = bytearray()
        deadline = time.monotonic() + 30
        try:
            while marker.encode() not in output:
                remaining = deadline - time.monotonic()
                if remaining <= 0 or not selector.select(remaining):
                    raise RuntimeError(f'{mode}: timed out; output={output.decode(errors="replace")}')
                chunk = os.read(child.stdout.fileno(), 4096)
                if not chunk:
                    raise RuntimeError(f'{mode}: worker exited before cut; output={output.decode(errors="replace")}')
                output.extend(chunk)
            os.kill(child.pid, signal.SIGKILL)
            assert child.wait(timeout=10) == -signal.SIGKILL
        finally:
            selector.close()
            if child.poll() is None:
                child.kill()
                child.wait(timeout=10)
            child.stdin.close()
            child.stdout.close()
        env['GLADE_Q2_MODE'] = 'recover'
        recovered = subprocess.run([worker, '--exact', 'q2_process_worker', '--ignored', '--nocapture'],
                                   env=env, capture_output=True, timeout=30, check=True)
        assert b'Q2_RECOVERED 3 23' in recovered.stdout, recovered.stdout
        print(f'{mode}: SIGKILL then fresh-process replay/exact retry PASS')
