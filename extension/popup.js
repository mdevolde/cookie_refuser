document.addEventListener("DOMContentLoaded", () => {
    const wordList = document.getElementById("word-list");
    const saveButton = document.getElementById("save");
    const status = document.getElementById("status");
    const resetButton = document.getElementById("reset");

    chrome.storage.local.get(["words"], (result) => {
        if (result.words) {
            wordList.value = result.words.join("\n");
        }
    });

    saveButton.addEventListener("click", () => {
        const words = wordList.value.split("\n").map((word) => word.trim()).filter((word) => word !== "");
        chrome.storage.local.set({ words }, () => {
            status.textContent = "Mots enregistrés !";
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
});
