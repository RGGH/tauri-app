const { invoke } = window.__TAURI__.tauri;

let greetInputEl;
let greetMsgEl;

async function greet() {
  greetMsgEl.textContent = await invoke("greet", { name: greetInputEl.value });
}

window.addEventListener("DOMContentLoaded", () => {
  greetInputEl = document.querySelector("#greet-input");
  greetMsgEl = document.querySelector("#greet-msg");

  document.querySelector("#greet-form").addEventListener("submit", (e) => {
    e.preventDefault();
    greet();
  });

  // ✅ Call `my_custom_command` when the "call-rust" button is clicked
  document.getElementById("call-rust").addEventListener("click", () => {
    invoke("my_custom_command").then(() => {
      console.log("Rust function was called");
    });
  });
});

