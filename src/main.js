const { invoke } = window.__TAURI__.tauri;

let v1ResultEl;
let v2ResultEl;

async function search() {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  const market = document.getElementById('market').value;
  const shopId = document.getElementById('shopId').value;
  const result = await invoke("search", { shopId: shopId, market: market });

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

window.addEventListener("DOMContentLoaded", () => {
  v1ResultEl = document.querySelector("#v1Result");
  v2ResultEl = document.querySelector("#v2Result");
  document
    .querySelector("#searchButton")
    .addEventListener("click", () => search());
});
