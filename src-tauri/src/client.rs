use rig::{completion::Prompt, providers::openai};


pub async fn llm_get(invoke:String)->String {
    // Create OpenAI client and model
    // This requires the `OPENAI_API_KEY` environment variable to be set.
    let openai_client = openai::Client::from_env();
    let gpt = openai_client.agent("gpt-3.5-turbo").build();


    // Prompt the model and print its response
    let response = gpt
        .prompt(invoke)
        .await
        .expect("Failed to prompt GPT");

    println!("GPT-3.5-turbo: {response}");
    response
}
