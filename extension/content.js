(async () => {
    const wasmModule = await import(chrome.runtime.getURL("wasm/cookie_refuser.js"));
    const { default: init, run } = wasmModule;
    await init();
    run();
})();