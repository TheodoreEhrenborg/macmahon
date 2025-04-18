mod calculator;

use calculator::Calculator;
use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    let mut calculator = Calculator::new();
    
    // Read from stdin
    let stdin = io::stdin();
    let reader = stdin.lock();

    // Process each line from stdin
    for line_result in reader.lines() {
        let line = line_result?;
        let processed_line = calculator.evaluate(&line);
        
        // Write the processed line to stdout
        println!("{}", processed_line);
    }

    Ok(())
}
