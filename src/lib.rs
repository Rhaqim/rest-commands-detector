use neon::prelude::*;
use regex::Regex;
use std::fs;

fn detect_methods_in_file(mut cx: FunctionContext) -> JsResult<JsString> {
    let file_path = cx.argument::<JsString>(0)?.value(&mut cx);
    let content = fs::read_to_string(file_path).expect("Unable to read file");

    // Update patterns to match method and endpoint
    let re = Regex::new(r#"\b(POST|GET|DELETE|PATCH|PUT)\s*\(\s*\"([^\"]+)\""#).unwrap();

    let mut curl_commands = Vec::new();

    for caps in re.captures_iter(&content) {
        let method = caps.get(1).map_or("", |m| m.as_str());
        let endpoint = caps.get(2).map_or("", |e| e.as_str());

        // Construct the curl command
        let curl_command = format!("\n{} http://localhost:8080{}\n", method, endpoint);
        curl_commands.push(curl_command);
    }

    if curl_commands.is_empty() {
        Ok(cx.string("No HTTP methods detected."))
    } else {
        let result = curl_commands.join("\n###\n");
        Ok(cx.string(result))
    }
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("detectMethodsInFile", detect_methods_in_file)?;
    Ok(())
}
