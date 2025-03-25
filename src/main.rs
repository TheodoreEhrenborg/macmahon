use rhai::{Dynamic, Engine};
use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};

fn main() -> io::Result<()> {
    // Get filename from command line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <filename>", args[0]);
        std::process::exit(1);
    }
    let filename = &args[1];

    // Initialize Rhai engine
    let engine = Engine::new();

    // Open input file
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    // Create output file with .calc extension
    let output_filename = format!("{}.calc", filename);
    let mut output_file = File::create(output_filename)?;

    // Process each line
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

        // Write the processed line to output file
        writeln!(output_file, "{}", processed_line)?;
    }

    println!("Processing complete. Output saved to {}.calc", filename);

    Ok(())
}
