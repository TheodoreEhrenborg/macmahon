use buckingham::u;
use buckingham::Unit;
use rhai::EvalAltResult;
use rhai::Scope;
use rhai::{Dynamic, Engine};
use std::io::{self, BufRead};

fn wrapped_u(input: &str) -> Result<Unit, Box<EvalAltResult>> {
    u(input).map_err(|_| "Whoops!".into())
}

fn main() -> io::Result<()> {
    // Initialize Rhai engine
    let mut engine = Engine::new();

    engine.register_fn("m", wrapped_u);

    // Read from stdin
    let stdin = io::stdin();
    let reader = stdin.lock();

    let mut scope = Scope::new();

    // Process each line from stdin
    for line_result in reader.lines() {
        let line = line_result?;

        // Check if the line is empty or contains only whitespace
        if line.trim().is_empty() {
            println!("{}", line);
            continue;
        }

        // Extract the expression (everything before "=>" if it exists)
        let expression = match line.find("=>") {
            Some(pos) => line[..pos].trim(),
            None => line.trim(),
        };

        // Evaluate the expression
        let processed_line = match engine.eval_with_scope::<Dynamic>(&mut scope, expression) {
            Ok(result) => format!("{} => {}", expression, result),
            Err(_) => line.clone(), // Keep original line if evaluation fails
        };

        // Write the processed line to stdout
        println!("{}", processed_line);
    }

    Ok(())
}
