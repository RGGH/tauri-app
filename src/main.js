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

	document.getElementById("call-rust2").addEventListener("click", () => {
  invoke("my_custom_command2", { invokeMessage: "Moo" }).then((response) => {
    console.log("Rust function2 was called with response:", response);
  });
  });
});

