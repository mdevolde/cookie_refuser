document.addEventListener("DOMContentLoaded", () => {
    const wordList = document.getElementById("word-list");
    const maxClicksInput = document.getElementById("max-clicks");
    const saveButton = document.getElementById("save");
    const resetButton = document.getElementById("reset");
    const toggle = document.getElementById('disable-extension-toggle')
    const status = document.getElementById("status");

    chrome.storage.local.get(["words", "maxClicks", "extensionEnabled"], (result) => {
        if (result.words) {
            wordList.value = result.words.join("\n");
        }
        if (result.maxClicks) {
            maxClicksInput.value = result.maxClicks;
        }
        toggle.checked = result.extensionEnabled !== undefined ? result.extensionEnabled : true;
    });

    saveButton.addEventListener("click", () => {
        const words = wordList.value.split("\n").map((word) => word.trim()).filter((word) => word !== "");
        const maxClicks = parseInt(maxClicksInput.value, 10) || 200;
        chrome.storage.local.set({ words, maxClicks }, () => {
            status.textContent = "Configuration saved!";
            setTimeout(() => (status.textContent = ""), 2000);
        });
    });

    resetButton.addEventListener("click", () => {
        chrome.tabs.query({ active: true, currentWindow: true }, (tabs) => {
            if (tabs[0].id) {
                chrome.tabs.sendMessage(
                    tabs[0].id,
                    { action: "reset_counter" },
                    handleResetResponse
                );
            }
        });
    });

    function handleResetResponse(response) {
        if (response?.status) {
            status.textContent = response.status;
            setTimeout(() => (status.textContent = ""), 2000);
        } else {
            status.textContent = "Unsuccessful reset!";
            setTimeout(() => (status.textContent = ""), 2000);
        }
    }

    toggle.addEventListener('change', () => {
        const isEnabled = toggle.checked;
        chrome.storage.local.set({ extensionEnabled: isEnabled }, () => {
            document.getElementById('status').textContent = "Status saved!";
            setTimeout(() => (status.textContent = ""), 2000);
        });
    });
});
