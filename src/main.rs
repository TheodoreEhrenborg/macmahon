use rhai::Dynamic;
use rhai::Engine;
fn main() {
    let engine = Engine::new();
    let result: Dynamic = engine.eval_expression("3./10").unwrap();
    dbg!(result.to_string());
    dbg!(result);
}
