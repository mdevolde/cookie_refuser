(async () => {
    chrome.tabs.onUpdated.addListener((tabId, changeInfo, tab) => {
        chrome.storage.local.get(["extensionEnabled"], async (result) => {
            const isEnabled = result.extensionEnabled !== undefined ? result.extensionEnabled : true;
            if (isEnabled && changeInfo.status === 'complete' && tab.url) {
                chrome.scripting.executeScript({
                    target: { tabId },
                    files: ["content.js"],
                })
                .catch((_) => {});
            }
        });
    });
})();