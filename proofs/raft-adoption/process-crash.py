#!/usr/bin/env python3
"""Q2 actual SIGKILL with original complete receipts retained by the parent."""
import argparse
import os
from pathlib import Path
import selectors
import signal
import subprocess
import tempfile
import time
import unittest

# Independent specification for committed mutation command(2), before any worker
# output: request fields, original index, Accepted tag, all Resource fields.
EXPECTED_MUTATION = (7, 100, 1, 1, 2, 3, 0, 100, 40, 1, 1, 1, 23, 0)


def compare_receipts(expected, lookup, retry, materialized_payload=23):
    return lookup == expected and retry == expected and materialized_payload == 23


def receipt_records(output, kind):
    prefix = f'Q2_RECEIPT {kind} '.encode()
    records = []
    for line in output.splitlines(keepends=True):
        if line.startswith(prefix) and line.endswith(b'\n'):
            tokens = line[len(prefix):].strip().split()
            try:
                values = tuple(int(token) for token in tokens)
            except ValueError as error:
                raise RuntimeError('non-numeric receipt') from error
            if any(value < 0 or value > (1 << 64) - 1 or str(value).encode() != token
                   for value, token in zip(values, tokens)):
                raise RuntimeError('noncanonical numeric receipt')
            if len(values) < 7:
                raise RuntimeError('incomplete receipt')
            if values[6] == 0:
                if len(values) != 14 or values[13] not in (0, 1):
                    raise RuntimeError('incomplete Accepted Resource')
            elif values[6] == 1:
                if len(values) != 8 or values[7] > 12:
                    raise RuntimeError('invalid rejection code')
            else:
                raise RuntimeError('unknown outcome tag')
            records.append(values)
    return records


def only_receipt(output, kind):
    records = receipt_records(output, kind)
    if len(records) != 1:
        raise RuntimeError(f'expected one {kind} receipt; output={output!r}')
    return records[0]


class ReceiptOracleRegression(unittest.TestCase):
    def test_home_generation_mutant_is_rejected(self):
        for field in [10, 11]:
            with self.subTest(field=field):
                mutant = list(EXPECTED_MUTATION)
                mutant[field] += 1
                self.assertFalse(compare_receipts(EXPECTED_MUTATION, tuple(mutant), tuple(mutant)))

    def test_accepted_to_rejected_mutant_is_rejected(self):
        mutant = (*EXPECTED_MUTATION[:6], 1, 0)
        self.assertFalse(compare_receipts(EXPECTED_MUTATION, mutant, mutant))

    def test_exact_original_receipt_is_accepted(self):
        self.assertTrue(compare_receipts(EXPECTED_MUTATION, EXPECTED_MUTATION, EXPECTED_MUTATION))

    def test_parser_preserves_full_resource_and_rejection(self):
        accepted = ('Q2_RECEIPT ACK ' + ' '.join(map(str, EXPECTED_MUTATION)) + '\n').encode()
        self.assertEqual(only_receipt(accepted, 'ACK'), EXPECTED_MUTATION)
        rejected = (*EXPECTED_MUTATION[:6], 1, 8)
        line = ('Q2_RECEIPT LOOKUP ' + ' '.join(map(str, rejected)) + '\n').encode()
        self.assertEqual(only_receipt(line, 'LOOKUP'), rejected)
        with self.assertRaises(RuntimeError):
            only_receipt(accepted.rstrip(b'\n'), 'ACK')
        with self.assertRaises(RuntimeError):
            only_receipt(accepted + accepted, 'ACK')
        with self.assertRaises(RuntimeError):
            only_receipt(b'Q2_RECEIPT ACK 7 100\n', 'ACK')


def run(worker):
    for mode in ['write-ack', 'write-cut']:
        with tempfile.TemporaryDirectory(prefix='glade-q2-kill-') as directory:
            env = {'GLADE_Q2_ROOT': directory, 'GLADE_Q2_MODE': mode}
            child = subprocess.Popen(
                [worker, '--exact', 'q2_process_worker', '--ignored', '--nocapture'],
                env=env, stdin=subprocess.PIPE, stdout=subprocess.PIPE, stderr=subprocess.STDOUT)
            selector = selectors.DefaultSelector()
            selector.register(child.stdout, selectors.EVENT_READ)
            output = bytearray()
            deadline = time.monotonic() + 30
            try:
                while True:
                    if mode == 'write-ack' and receipt_records(output, 'ACK'):
                        # Keep the actual complete pre-kill receipt outside the
                        # terminated process; never infer it from later replay.
                        expected = only_receipt(output, 'ACK')
                        if expected != EXPECTED_MUTATION:
                            raise RuntimeError(f'pre-kill receipt violates fixture: {expected}')
                        break
                    if mode == 'write-cut' and b'Q2_COMMIT_BEFORE_APPLY\n' in output.splitlines(keepends=True):
                        # No receipt exists at this cut: independent full fixture.
                        expected = EXPECTED_MUTATION
                        break
                    remaining = deadline - time.monotonic()
                    if remaining <= 0 or not selector.select(remaining):
                        raise RuntimeError(f'{mode}: timed out; output={output.decode(errors="replace")}')
                    chunk = os.read(child.stdout.fileno(), 4096)
                    if not chunk:
                        raise RuntimeError(f'{mode}: worker exited before cut; output={output.decode(errors="replace")}')
                    output.extend(chunk)
                os.kill(child.pid, signal.SIGKILL)
                if child.wait(timeout=10) != -signal.SIGKILL:
                    raise RuntimeError('worker did not terminate by SIGKILL')
            finally:
                selector.close()
                if child.poll() is None:
                    child.kill()
                    child.wait(timeout=10)
                child.stdin.close()
                child.stdout.close()
            env['GLADE_Q2_MODE'] = 'recover'
            recovered = subprocess.run(
                [worker, '--exact', 'q2_process_worker', '--ignored', '--nocapture'],
                env=env, capture_output=True, timeout=30, check=True)
            lookup = only_receipt(recovered.stdout, 'LOOKUP')
            retry = only_receipt(recovered.stdout, 'RETRY')
            if not compare_receipts(expected, lookup, retry):
                raise RuntimeError(f'{mode}: original={expected}, lookup={lookup}, retry={retry}')
            print(f'{mode}: SIGKILL then fresh-process complete original receipt/exact retry PASS')


if __name__ == '__main__':
    parser = argparse.ArgumentParser()
    parser.add_argument('--self-test', action='store_true')
    parser.add_argument('--worker', type=Path, help='compiled process_crash integration-test executable')
    args = parser.parse_args()
    if args.self_test:
        unittest.main(argv=['receipt-oracle'], exit=True)
    if args.worker is None:
        parser.error('--worker is required unless --self-test is used')
    run(str(args.worker.resolve()))
