// Cold-join regression: isolated nodes and temporary data, never the running desk.
import assert from 'node:assert/strict';
import { execFileSync, spawn } from 'node:child_process';
import { mkdtempSync, readFileSync, writeFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join } from 'node:path';
import { fileURLToPath } from 'node:url';
import { createInterface } from 'node:readline';
import { loadSchema } from '../glade/client-ts/src/taut/schema.ts';
import { GladeClient } from '../glade/client-ts/src/client.ts';
import { utf8 } from '../glade/client-ts/src/bytes.ts';

const bin = fileURLToPath(new URL('../glade/node/target/debug/glade-node', import.meta.url));
const schema = loadSchema(JSON.parse(readFileSync(new URL('../taut/corpus/glade.ir.json', import.meta.url), 'utf8')));
const root = mkdtempSync(join(tmpdir(), 'gyld-takeover-repro-'));
const children = [];
const clients = [];
const key = utf8('self:probe');
const share = 'probe-settings';
const surface = 'probe.appearance';
const environment = { PATH: process.env.PATH, GLADE_HOME: root };
const id = (name, kind) => execFileSync(bin, [kind, '--name', name], {env: environment, encoding:'utf8'}).trim();
const keys = Object.fromEntries(['a','b'].map(name => [name, {endpoint:id(name,'endpoint-id'), node:id(name,'node-id')}]));
const app = join(root,'app.glade');
writeFileSync(app, `glade-app v1\napp probe\nbinding ${surface} value share private latest\nworkspace ${share} probe\nseed ${keys.a.node} ${share} read.*,write.*\nseed ${keys.b.node} ${share} read.*,write.*\n`);
const timeout = setTimeout(() => { console.error('Diagnostic timed out'); for (const child of children) { child.kill('SIGTERM'); } process.exit(1); }, 25000);

async function start(name, peer) {
  const child = spawn(bin, ['--profile','local','--name',name,'--app',app,'--peer',peer,'0'], {env:environment, stdio:['ignore','pipe','pipe']});
  children.push(child);
  let endpoint;
  let errors = '';
  child.stderr.on('data', bytes => { errors += bytes; });
  return await new Promise((resolve, reject) => {
    createInterface({input:child.stdout}).on('line', line => {
      const peerLine = /^peer \S+ (\S+)$/.exec(line);
      if (peerLine) { endpoint = peerLine[1]; }
      const listen = /^listening (\d+)$/.exec(line);
      if (listen) { resolve({port:Number(listen[1]), endpoint}); }
    });
    child.on('exit', code => { reject(new Error(`Node ${name} exited ${code}: ${errors}`)); });
  });
}
async function client(port, name) {
  const result = new GladeClient(schema, name);
  clients.push(result);
  await result.connect(`ws://127.0.0.1:${port}`);
  assert.equal((await result.subscribeOutcome(share, surface, key)).ok, true);
  return result;
}
function value(client) {
  const raw = client.fold(share, surface, 'value', key);
  return raw === null ? null : JSON.parse(new TextDecoder().decode(raw));
}
async function holds(client, expected) {
  const until = Date.now() + 5000;
  while (Date.now() < until) {
    if (JSON.stringify(value(client)) === JSON.stringify(expected)) { return; }
    await new Promise(resolve => setTimeout(resolve, 20));
  }
  assert.deepEqual(value(client), expected);
}

try {
  const a = await start('a', keys.b.endpoint);
  const old = await client(a.port, 'established-page');
  for (let n=0; n<10; n++) {
    assert.equal((await old.appendOutcome(share,surface,'value',utf8(JSON.stringify({theme:'solar',revision:n})),key)).ok,true);
  }
  const b = await start('b', `${keys.a.endpoint}@${a.endpoint}`);
  const newcomer = await client(b.port, 'new-page');
  console.log('A value before B joins:', JSON.stringify(value(old)));
  await holds(newcomer, {theme:'solar',revision:9});
  console.log('B value after joining:', JSON.stringify(value(newcomer)));
  const update = await newcomer.appendOutcome(share,surface,'value',utf8(JSON.stringify({theme:'nord',revision:0})),key);
  console.log('B setting write accepted:',update.ok);
  // Fresh readers eliminate React, IndexedDB and stale browser subscriptions.
  const readA = await client(a.port,'read-a');
  const readB = await client(b.port,'read-b');
  await Promise.all([readA, readB, old].map(c => holds(c, {theme:'nord',revision:0})));
  console.log('Fresh read through A:',JSON.stringify(value(readA)));
  console.log('Fresh read through B:',JSON.stringify(value(readB)));
  assert.deepEqual(value(readA),value(readB),'existing settings converge through the original owner');
  console.log('PASS: existing settings and a follower update converge after cold join');
  console.log('Diagnostic data:',root);
} finally {
  clearTimeout(timeout);
  for (const c of clients) { c.close(); }
  for (const child of children) { child.kill('SIGTERM'); }
}
