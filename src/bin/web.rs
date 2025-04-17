use web_sys::HtmlTextAreaElement;
use yew::prelude::*;

#[function_component(TextEditor)]
fn text_editor() -> Html {
    // Create state for the textarea content
    let value = use_state(|| "I am writing a long story...".to_string());

    // Display value (uppercase version)
    let display_value = value.to_uppercase();

    // Callback for handling text input changes
    let on_text_input = {
        let value = value.clone();

        Callback::from(move |e: InputEvent| {
            let input: HtmlTextAreaElement = e.target_unchecked_into::<HtmlTextAreaElement>();
            value.set(input.value());
        })
    };

    html! {
        <div class="content-area">
            <textarea value={(*value).clone()} oninput={on_text_input} />
            <div class="display-area">
                {display_value}
            </div>
        </div>
    }
}

fn main() {
    yew::Renderer::<TextEditor>::new().render();
}
