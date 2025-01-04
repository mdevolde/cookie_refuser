(async () => {
    chrome.storage.local.get(["words"], async (result) => {
        const defaultWords = ["refuser", "rejeter", "reject", "decline", "refuse", "optional", "necessary", "essential", "options", "enregistrer"];
        const wordlist = (result.words || []).concat(defaultWords);

        const wasmModule = await import(chrome.runtime.getURL("wasm/cookie_refuser.js"));
        const { default: init, run } = wasmModule;
        await init();
        run(wordlist);
    });
})();