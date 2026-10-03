// Live smoke probe: only a unique, otherwise unused session key is written.
// No existing desktop layout or appearance value is changed.
import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import { randomUUID } from 'node:crypto';
import { loadSchema } from '../glade/client-ts/src/taut/schema.ts';
import { GladeClient } from '../glade/client-ts/src/client.ts';
import { utf8, hex } from '../glade/client-ts/src/bytes.ts';

const schema = loadSchema(JSON.parse(readFileSync(
  new URL('../taut/corpus/glade.ir.json', import.meta.url), 'utf8',
)));
const run = randomUUID();
const key = utf8(`self:owner/__two-node-smoke-${run}`);
const share = 'ws-razel';
const surface = 'gyld.desk';
const a = new GladeClient(schema, `gyld-smoke-a-${run}`);
const b = new GladeClient(schema, `gyld-smoke-b-${run}`);
const deadline = setTimeout(() => {
  console.error('FAIL: two-node probe exceeded 25 seconds');
  a.close(); b.close(); process.exit(1);
}, 25000);

async function arrives(client, value) {
  const end = Date.now() + 10000;
  while (Date.now() < end) {
    const actual = client.fold(share, surface, 'value', key);
    if (actual !== null && hex(actual) === hex(value)) {
      return;
    }
    await new Promise((resolve) => setTimeout(resolve, 25));
  }
  throw new Error('Remote value did not arrive');
}

try {
  await a.connect('ws://127.0.0.1:9099');
  await b.connect('ws://127.0.0.1:9106');
  await a.hello('owner');
  await b.hello('owner');
  assert.equal((await a.subscribeOutcome(share, surface, key)).ok, true);
  assert.equal((await b.subscribeOutcome(share, surface, key)).ok, true);
  const first = utf8(JSON.stringify({ probe: run, direction: 'A to B' }));
  assert.equal((await a.appendOutcome(share, surface, 'value', first, key)).ok, true);
  await arrives(b, first);
  console.log('PASS: A (9099) -> B (9106), acknowledged and received');
  const second = utf8(JSON.stringify({ probe: run, direction: 'B to A' }));
  assert.equal((await b.appendOutcome(share, surface, 'value', second, key)).ok, true);
  await arrives(a, second);
  console.log('PASS: B (9106) -> A (9099), acknowledged and received');
  console.log(`Probe key: self:owner/__two-node-smoke-${run}`);
} finally {
  clearTimeout(deadline);
  a.close(); b.close();
}
