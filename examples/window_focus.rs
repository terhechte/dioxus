#![allow(non_snake_case)]
use dioxus::prelude::*;

fn main() {
    dioxus_desktop::launch(app);
}

fn app(cx: Scope) -> Element {
    let value = use_state(cx, || false);

    cx.render(rsx! {
        div {
            width: "100%",
            height: "100%",
            display: "flex",
            flex_direction: "column",
            align_items: "center",
            {
                if *value.get() {
                    rsx!(Example {
                        value: 10
                    })
                } else {
                    rsx!(Example {
                        value: 20
                    })
                }
            }
            button {
                onclick: move |_| value.set(!*value.get()),
                "Toggle (current is {value})"
            }
        }
    })
}

#[inline_props]
fn Example(cx: Scope, value: usize) -> Element {
    let state = use_state(cx, || *value);
    cx.render(rsx!(h3 {
        "Value is {state}"
    }))
}
