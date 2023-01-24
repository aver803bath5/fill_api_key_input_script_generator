const {invoke} = window.__TAURI__.tauri;

let v1ResultEl;
let v2ResultEl;

async function search() {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    const market = document.getElementById('market').value;
    const shopId = document.getElementById('shopId').value;
    const result = await invoke("search", {shopId: shopId, market: market});

    // Show the not found message if the result is null
    if (result === null) {
        v1ResultEl.textContent = "No result";
        v2ResultEl.textContent = "No result";
        return;
    }

    v1ResultEl.textContent = `var tokenInput = document.querySelector('input[name=token]');
tokenInput.focus();
document.execCommand('inserttext', false, '${result.token}');

var apiKeyInput = document.querySelector('input[name=apikey]');
apiKeyInput.focus();
document.execCommand('inserttext', false, '${result.api_key}');

var saltKeyInput = document.querySelector('input[name=saltkey]');
saltKeyInput.focus();
document.execCommand('inserttext', false, '${result.salt}');`;

    v2ResultEl.textContent = `var tokenInput = document.querySelector('input#Token');
tokenInput.focus();
document.execCommand('inserttext', false, '${result.token}');

var apiKeyInput = document.querySelector('input#ApiKey');
apiKeyInput.focus();
document.execCommand('inserttext', false, '${result.api_key}');

var saltKeyInput = document.querySelector('input#SaltKey');
saltKeyInput.focus();
document.execCommand('inserttext', false, '${result.salt}');`;
}

function showStatus(statusEl) {
    // show a success message
    statusEl.textContent = '複製成功';
    // hide the success message after 3 seconds
    setTimeout(() => {
        statusEl.textContent = '';
    }, 2000)
}

window.addEventListener("DOMContentLoaded", () => {
    v1ResultEl = document.querySelector("#v1Result");
    v2ResultEl = document.querySelector("#v2Result");
    document
        .querySelector("#searchButton")
        .addEventListener("click", () => search());

    document
        .querySelector('#v1CopyButton')
        .addEventListener('click', function () {
            // copy the text in the v1Result element
            navigator.clipboard.writeText(v1ResultEl.textContent).then(r => {
                showStatus(document.querySelector('#v1CopyStatus'));
            });
        });

    document
        .querySelector('#v2CopyButton')
        .addEventListener('click', () => {
            navigator.clipboard.writeText(v2ResultEl.textContent).then(r => {
                showStatus(document.querySelector('#v2CopyStatus'));
            });
        });
});
