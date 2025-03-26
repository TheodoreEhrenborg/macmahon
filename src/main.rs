use rhai::{Dynamic, Engine};
use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    // Initialize Rhai engine
    let engine = Engine::new();

    // Read from stdin
    let stdin = io::stdin();
    let reader = stdin.lock();

    // Process each line from stdin
    for line_result in reader.lines() {
        let line = line_result?;

        // Extract the expression (everything before "=>" if it exists)
        let expression = match line.find("=>") {
            Some(pos) => line[..pos].trim(),
            None => line.trim(),
        };

        // Evaluate the expression
        let processed_line = match engine.eval_expression::<Dynamic>(expression) {
            Ok(result) => format!("{} => {}", expression, result),
            Err(_) => line.clone(), // Keep original line if evaluation fails
        };

        // Write the processed line to stdout
        println!("{}", processed_line);
    }

    Ok(())
}
