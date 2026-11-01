import init,{replay} from './pkg/rune_web.js';
try{await init({module_or_path:new URL('./pkg/rune_web_bg.wasm',import.meta.url)});postMessage({type:'ready'});}catch(error){postMessage({type:'startup-error',message:String(error)});}
onmessage=({data})=>{if(data.type==='replay')try{postMessage({type:'result',id:data.id,canonical:replay(data.request)});}catch(error){postMessage({type:'compute-error',id:data.id,message:String(error)});}};
