(async () => {
    chrome.storage.local.get(["words"], async (result) => {
        const defaultWords = ["refuser", "rejeter", "reject", "decline", "refuse", "optional", "necessary", "essential", "options", "enregistrer"];
        const wordlist = (result.words || []).concat(defaultWords);

        const wasmModule = await import(chrome.runtime.getURL("wasm/cookie_refuser.js"));
        const { default: init, run } = wasmModule;
        await init();
        run(wordlist);
    });

    chrome.runtime.onMessage.addListener((message, _, sendResponse) => {
        if (message.action === "reset_counter") {
            window.localStorage.removeItem("cookie_refuser_click_count");
            sendResponse({ status: "Counter has been reset!" });
        }
    });
})();