use web_sys::HtmlTextAreaElement;
use yew::prelude::*;

// Import the calculator module
use crate::calculator::Calculator;

#[function_component(TextEditor)]
fn text_editor() -> Html {
    // Create state for the textarea content
    let value = use_state(|| "I am writing a long story...".to_string());
    
    // Create a calculator instance
    let calculator = use_mut_ref(|| Calculator::new());
    
    // Process the input through the calculator
    let display_value = {
        let mut calc = calculator.borrow_mut();
        let input_lines = (*value).split('\n');
        let processed_lines: Vec<String> = input_lines
            .map(|line| calc.evaluate(line))
            .collect();
        processed_lines.join("\n")
    };

    // Callback for handling text input changes
    let on_text_input = {
        let value = value.clone();

        Callback::from(move |e: InputEvent| {
            let input: HtmlTextAreaElement = e.target_unchecked_into::<HtmlTextAreaElement>();
            value.set(input.value());
        })
    };

    html! {
        <div class="content-area" style="display: flex; gap: 20px;">
            <textarea
                value={(*value).clone()}
                oninput={on_text_input}
                style="flex: 1; min-height: 200px;"
            />
            <div
                class="display-area"
                style="flex: 1; padding: 8px; border: 1px solid #ccc; background-color: #f9f9f9; white-space: pre-wrap;"
            >
                {display_value}
            </div>
        </div>
    }
}

fn main() {
    yew::Renderer::<TextEditor>::new().render();
}
