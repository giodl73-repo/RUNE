import {test,expect} from '@playwright/test';
import {readFile,mkdir} from 'node:fs/promises';
const fixtures=JSON.parse(await readFile('.cache/native-references.json','utf8'));
async function open(page){await page.goto('./');await expect(page.locator('#results')).toBeVisible();}
async function download(page,id){const event=page.waitForEvent('download');await page.locator('#'+id).click();return readFile(await(await event).path(),'utf8');}
async function supply(page,request){await page.locator('#request').fill(request.input_json);await page.locator('#kind').selectOption(request.kind);await page.locator('#profile').selectOption(request.profile);await page.locator('#run').click();}
test('all native responses match actual WASM exactly',async({page})=>{
 await open(page);const actual=await page.evaluate(async fixtures=>{const module=await import('./pkg/rune_web.js');await module.default();return fixtures.map(f=>module.replay(f.input));},fixtures);for(let i=0;i<fixtures.length;i++)expect(actual[i],fixtures[i].name).toBe(fixtures[i].canonical);
});
test('own descriptor exports exact input and native artifact',async({page})=>{
 await open(page);const req=JSON.parse(fixtures.find(f=>f.name==='data-contract').input);req.input_json=req.input_json.replaceAll('sample.entity','my.own.entity');await supply(page,req);await expect(page.locator('#artifact')).toContainText('my.own.entity');expect(await download(page,'submitted-download')).toBe(req.input_json);const report=JSON.parse(await download(page,'report-download'));expect(await download(page,'artifact-download')).toBe(report.artifact_json);expect(await download(page,'check-download')).toBe(report.check_json);await expect(page.locator('#inventory-download')).toBeDisabled();
});
test('own collection exports inventory and existing review packet',async({page})=>{
 await open(page);const req=JSON.parse(fixtures.find(f=>f.name==='collection').input);req.input_json=req.input_json.replaceAll('sample.collection','own.collection');await supply(page,req);await expect(page.locator('#inventory')).toContainText('own.collection');const report=JSON.parse(await download(page,'report-download'));expect(await download(page,'review-download')).toBe(report.review.artifact_json);expect(await download(page,'inventory-download')).toBe(report.inventory_json);
});
test('adapter refusal preserves compatible profile generation',async({page})=>{
 await open(page);await page.locator('[data-example="adapter-limit"]').click();await page.locator('#run').click();await expect(page.locator('#review')).toContainText('cannot represent invariants');await expect(page.locator('#artifact-download')).toBeEnabled();await expect(page.locator('#review-download')).toBeDisabled();
});
test('unknown fields are disclosed while original large integer is retained',async({page})=>{
 await open(page);await page.locator('[data-example="unknown-fields"]').click();await page.locator('#run').click();await expect(page.locator('#ignored')).toContainText('unrecognized');expect(await download(page,'original-download')).toContain('18446744073709551615');const report=JSON.parse(await download(page,'report-download'));expect(report.normalized_json).not.toContain('unrecognized');
});
test('invalid member is rejected and input retained',async({page})=>{
 await open(page);const req=JSON.parse(fixtures.find(f=>f.name==='blank-member').input);await supply(page,req);await expect(page.locator('#error')).toContainText('identity is missing');await expect(page.locator('#results')).toBeHidden();expect(await download(page,'original-download')).toBe(req.input_json);
});
test('input edits and profile changes invalidate exports',async({page})=>{
 await open(page);await page.locator('#profile').selectOption('rune.data_contract_json');await expect(page.locator('#results')).toBeHidden();await expect(page.locator('#artifact-download')).toBeDisabled();await page.locator('#run').click();await expect(page.locator('#results')).toBeVisible();await page.locator('#request').fill('{broken');await expect(page.locator('#results')).toBeHidden();
});
test('local CRLF input is exported and submitted byte exactly',async({page})=>{
 await open(page);const req=JSON.parse(fixtures.find(f=>f.name==='original-crlf').input);await page.locator('#request-file').setInputFiles({name:'own.json',mimeType:'application/json',buffer:Buffer.from(req.input_json)});expect(await download(page,'original-download')).toBe(req.input_json);await page.locator('#run').click();await expect(page.locator('#results')).toBeVisible();expect(await download(page,'submitted-download')).toBe(req.input_json);
});
test('invalid UTF-8 and oversized replacement retain prior draft',async({page})=>{
 await open(page);const before=await page.locator('#request').inputValue();await page.locator('#request-file').setInputFiles({name:'bad.json',mimeType:'application/json',buffer:Buffer.from([0xc3,0x28])});await expect(page.locator('#error')).toContainText('valid UTF-8');expect(await download(page,'original-download')).toBe(before);await page.locator('#request-file').setInputFiles({name:'large.json',mimeType:'application/json',buffer:Buffer.alloc(131073,32)});await expect(page.locator('#error')).toContainText('128 KiB');expect(await download(page,'original-download')).toBe(before);
});
test('submitted markup and control characters render as text',async({page})=>{
 await open(page);await supply(page,JSON.parse(fixtures.find(f=>f.name==='unicode-control').input));await expect(page.locator('#artifact')).toContainText('<img src=x onerror=alert(1)>');expect(await page.locator('#results img').count()).toBe(0);
});
test('late file cannot overwrite edit or reset',async({page})=>{
 await open(page);await page.evaluate(()=>{File.prototype.arrayBuffer=()=>new Promise(resolve=>window.finishRead=()=>resolve(new TextEncoder().encode('stale').buffer));});await page.locator('#request-file').setInputFiles({name:'held.json',mimeType:'application/json',buffer:Buffer.from('held')});await page.locator('#request').fill('newer draft');await page.evaluate(()=>window.finishRead());expect(await page.locator('#request').inputValue()).toBe('newer draft');await page.locator('#request-file').setInputFiles({name:'held.json',mimeType:'application/json',buffer:Buffer.from('held')});await page.locator('#reset').click();await page.evaluate(()=>window.finishRead());expect(await page.locator('#request').inputValue()).toContain('sample.entity');
});
test('file deadline retains draft and recovers checking',async({page})=>{
 await open(page);await page.clock.install();const before=await page.locator('#request').inputValue();await page.evaluate(()=>{File.prototype.arrayBuffer=()=>new Promise(()=>{});});await page.locator('#request-file').setInputFiles({name:'held.json',mimeType:'application/json',buffer:Buffer.from('held')});await expect(page.locator('#run')).toBeDisabled();await page.clock.fastForward(5001);await expect(page.locator('#error')).toContainText('five seconds');await expect(page.locator('#run')).toBeEnabled();expect(await page.locator('#request').inputValue()).toBe(before);
});
test('startup deadline retains independent source and input downloads',async({page})=>{
 await page.addInitScript(()=>{window.Worker=class{postMessage(){}terminate(){}};});await page.clock.install();await page.goto('./');await page.locator('#request').fill('retained');await page.clock.fastForward(12001);await expect(page.locator('#retry')).toBeVisible();expect(await download(page,'original-download')).toBe('retained');await expect(page.locator('a[href="./source.zip"]')).toBeVisible();
});
test('compute deadline fences late results',async({page})=>{
 await page.addInitScript(()=>{window.Worker=class{constructor(){window.fakeWorker=this;setTimeout(()=>this.onmessage?.({data:{type:'ready'}}),20);}postMessage(data){this.last=data;}terminate(){}};});await page.clock.install();await page.goto('./');await page.clock.fastForward(21);await expect(page.locator('#run')).toBeDisabled();await page.locator('#request').fill('retained');await page.clock.fastForward(10001);await expect(page.locator('#retry')).toBeVisible();await page.evaluate(()=>window.fakeWorker.onmessage({data:{id:window.fakeWorker.last.id,canonical:'{}'}}));await expect(page.locator('#results')).toBeHidden();expect(await download(page,'original-download')).toBe('retained');
});
test('stale computation cannot restore edited result',async({page})=>{
 await page.addInitScript(()=>{window.Worker=class{constructor(){window.fakeWorker=this;setTimeout(()=>this.onmessage?.({data:{type:'ready'}}),20);}postMessage(data){this.last=data;}terminate(){}};});await page.goto('./');await expect(page.locator('#status')).toContainText('Checking');await page.locator('#request').fill('newer');await page.evaluate(()=>window.fakeWorker.onmessage({data:{id:window.fakeWorker.last.id,canonical:JSON.stringify({ok:true})}}));await expect(page.locator('#results')).toBeHidden();await expect(page.locator('#run')).toBeEnabled();
});
test('failed WASM download can retry retained input',async({page})=>{
 await page.route('**/*.wasm',route=>route.abort());await page.goto('./');await expect(page.locator('#retry')).toBeVisible();const before=await page.locator('#request').inputValue();await page.unroute('**/*.wasm');await page.locator('#retry').click();await expect(page.locator('#results')).toBeVisible();expect(await page.locator('#request').inputValue()).toBe(before);
});
test('keyboard chooses collection and checks it',async({page})=>{
 await open(page);await page.locator('[data-example="collection"]').focus();await page.keyboard.press('Enter');await expect(page.locator('#request')).toBeFocused();await page.locator('#run').focus();await page.keyboard.press('Enter');await expect(page.locator('#inventory')).toContainText('sample.collection');
});
test('no JavaScript retains input and source ZIP',async({browser,baseURL})=>{
 const context=await browser.newContext({javaScriptEnabled:false});const page=await context.newPage();await page.goto(baseURL);await expect(page.locator('#static-help')).toBeVisible();await expect(page.locator('#request')).toContainText('sample.entity');expect((await(await page.request.get(new URL('source.zip',baseURL).href)).body()).subarray(0,2).toString()).toBe('PK');await context.close();
});
test('WASM MIME source and licenses are discoverable',async({request,baseURL})=>{
 const response=await request.get(new URL('pkg/rune_web_bg.wasm',baseURL).href);expect(response.headers()['content-type']).toContain('application/wasm');expect([...((await response.body()).subarray(0,4))]).toEqual([0,97,115,109]);for(const path of ['BUILD.md','LICENSE','THIRD-PARTY-NOTICES.txt','rust/COPYRIGHT-library.html','source/crates/rune-core/src/lib.rs','source/crates/rune-adapters/src/lib.rs','source/web/adapter/Cargo.lock'])expect((await request.get(new URL(path,baseURL).href)).ok(),path).toBeTruthy();expect(await(await request.get(new URL('LICENSE',baseURL).href)).text()).toContain('MIT License');const notices=await(await request.get(new URL('THIRD-PARTY-NOTICES.txt',baseURL).href)).text();expect(notices).toContain('serde_ignored');expect(notices).toContain('wasm-bindgen');
});
for(const width of [1440,390])test('readable results at '+width,async({page})=>{
 await page.setViewportSize({width,height:1000});await open(page);await page.locator('[data-example="collection"]').click();await page.locator('#run').click();await expect(page.locator('#inventory')).toContainText('sample.collection');expect(await page.evaluate(()=>document.documentElement.scrollWidth<=innerWidth)).toBeTruthy();await mkdir('.cache/review',{recursive:true});await page.screenshot({path:`.cache/review/results-${width}.png`,fullPage:true});
});
