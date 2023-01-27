use leptos::*;
use leptos_material::*;

/// A simple counter component.
///
/// You can use doc comments like this to document your component.
#[component]
pub fn SimpleCounter(
    cx: Scope,
    /// The starting value for the counter
    initial_value: i32,
    /// The change that should be applied each time the button is clicked.
    step: i32,
) -> impl IntoView {
    let (value, set_value) = create_signal(cx, initial_value);

    view! { cx,
        <MaterialColors primary=[0x67, 0x50, 0xA4] />
        <MaterialStyle/>
        <div style="display:flex">
            <Card filled=true>
                <img src="public/example.jpg"/>
                <hgroup>
                    <h2> "Title" </h2>
                    <p> "Subtitle" </p>
                </hgroup>
                <p>
                    "Just some normal text content."
                </p>
                <button on:click=move |_| set_value(0)>"Clear"</button>
                <button on:click=move |_| set_value.update(|value| *value -= step)>"-1"</button>
                <span>"Value: " {value} "!"</span>
                <button on:click=move |_| set_value.update(|value| *value += step)>"+1"</button>
            </Card>
            <For each=|| 0..5 key=|i| *i view=move |_| {
                view!{cx,
                    <Card elevated=true>
                        <hgroup>
                            <h2> "Title" </h2>
                            <p> "Subtitle" </p>
                        </hgroup>
                        <p>
                            "Just some normal text content."
                        </p>
                        <button on:click=move |_| set_value(0)>"Clear"</button>
                        <button on:click=move |_| set_value.update(|value| *value -= step)>"-1"</button>
                        <span>"Value: " {value} "!"</span>
                        <button on:click=move |_| set_value.update(|value| *value += step)>"+1"</button>
                    </Card>
                }}
            />
        </div>
    }
}

pub fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();
    mount_to_body(|cx| {
        view! { cx,
            <MaterialColors primary=[0x67, 0x50, 0xA4] />
            <MaterialStyle/>
            <h1>"Components"</h1>
            <h2>"Cards"</h2>
            <div style="display:flex">
            <Card elevated=true>
                "Some text"
            </Card>
            <Card filled=true>
                "Some text"
            </Card>
            <Card outlined=true>
                "Some text"
            </Card>
            </div>
        }
    })
}
