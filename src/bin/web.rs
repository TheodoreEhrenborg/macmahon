use web_sys::HtmlTextAreaElement;
use yew::prelude::*;

#[function_component(TextEditor)]
fn text_editor() -> Html {
    // Create state for the textarea content
    let value = use_state(|| "I am writing a long story...".to_string());

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
            <textarea value={value.to_string()} oninput={on_text_input} />
        </div>
    }
}

fn main() {
    yew::Renderer::<TextEditor>::new().render();
}
