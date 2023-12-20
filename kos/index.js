false && process.versions["nw-flavor"] === "sdk" && chrome.developerPrivate.openDevTools({
	renderViewId: -1,
	renderProcessId: -1,
	extensionId: chrome.runtime.id,
});

(async()=>{
    window.kash = await import('/app/wasm/kash.js');
    const wasm = await window.kash.default('/app/wasm/kash_bg.wasm');
    await window.kash.init_core();
})();
