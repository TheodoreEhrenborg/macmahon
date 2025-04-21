use buckingham::u;
use buckingham::Unit;
use rhai::{Dynamic, Engine, EvalAltResult, Scope};

fn wrapped_u(input: &str) -> Result<Unit, Box<EvalAltResult>> {
    u(input).map_err(|_| "Whoops!".into())
}

pub struct Calculator {
    engine: Engine,
    scope: Scope<'static>,
}

impl Calculator {
    pub fn new() -> Self {
        let mut engine = Engine::new();
        engine.register_fn("m", wrapped_u);

        Calculator {
            engine,
            scope: Scope::new(),
        }
    }

    pub fn evaluate(&mut self, input: &str) -> String {
        // Check if the line is empty or contains only whitespace
        if input.trim().is_empty() {
            return input.to_string();
        }

        // Extract the expression (everything before "=>" if it exists)
        let expression = match input.find("=>") {
            Some(pos) => input[..pos].trim(),
            None => input.trim(),
        };

        // Evaluate the expression
        match self
            .engine
            .eval_with_scope::<Dynamic>(&mut self.scope, expression)
        {
            Ok(result) => format!("{} => {}", expression, result),
            Err(_) => input.to_string(), // Keep original line if evaluation fails
        }
    }
}
