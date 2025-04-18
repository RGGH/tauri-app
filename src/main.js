const { invoke } = window.__TAURI__.tauri;
window.addEventListener("DOMContentLoaded", () => {
  const greetInputEl = document.querySelector("#greet-input");
  const greetMsgEl = document.querySelector("#greet-msg");

  // Listen for the form submission
  document.querySelector("#greet-form").addEventListener("submit", (e) => {
    e.preventDefault(); // Prevent default form submission

    const userInput = greetInputEl.value; // Get the input value

    // Pass the user input to the Rust function when the form is submitted
    invoke("ask_llm", { invokeMessage: userInput }).then((response) => {
      console.log("Rust function2 was called with response:", response);
      document.getElementById('result-display').textContent = response;
    });

    greet(); // Call any additional greeting logic if needed
  });

  // ✅ Call `my_custom_command` when the "call-rust" button is clicked
  document.getElementById("use-browser").addEventListener("click", () => {
    invoke("my_custom_command").then(() => {
      console.log("Rust function was called");
    });
  });
});
