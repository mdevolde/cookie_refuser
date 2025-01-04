document.addEventListener("DOMContentLoaded", () => {
    const wordList = document.getElementById("word-list");
    const saveButton = document.getElementById("save");
    const status = document.getElementById("status");

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
});
