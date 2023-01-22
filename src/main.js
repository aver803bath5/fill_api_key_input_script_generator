const { invoke } = window.__TAURI__.tauri;

let v1ResultEl;
let v2ResultEl;

async function greet() {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  v1ResultEl.textContent = await invoke("greet", { name: "abc" });
}

window.addEventListener("DOMContentLoaded", () => {
  v1ResultEl = document.querySelector("#v1Result");
  v2ResultEl = document.querySelector("#v2Result");
  document
    .querySelector("#searchButton")
    .addEventListener("click", () => greet());
});
