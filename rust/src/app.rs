use leptos::leptos_dom::ev::SubmitEvent;
use leptos::*;
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::to_value;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &JsValue);
    #[wasm_bindgen(catch, js_namespace = ["window", "__TAURI__", "notification"])]
    async fn isPermissionGranted() -> Result<JsValue, JsValue>; // boolean in JS
    #[wasm_bindgen(catch, js_namespace = ["window", "__TAURI__", "notification"])]
    async fn requestPermission() -> Result<JsValue, JsValue>; // type Permission = "granted" | "denied" | "default";
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "notification"])]
    fn sendNotification(args: JsValue);
}

#[derive(Serialize, Deserialize)]
struct ToastArgs<'a> {
    title: &'a str,
    body: &'a str,
}

#[component]
pub fn App() -> impl IntoView {
    let (toast_message, set_toast_message) = create_signal(String::new());

    let update_toast_message = move |ev| {
        let v = event_target_value(&ev);
        set_toast_message.set(v);
    };

    let toast_notification = move |ev: SubmitEvent| {
        ev.prevent_default();
        spawn_local(async move {
            let message = toast_message.get_untracked();
            if message.is_empty() {
                return;
            }

            let permissionGranted = isPermissionGranted().await.unwrap();
            if permissionGranted.to_owned().is_falsy() {
                let permission = requestPermission().await.unwrap();
                if permission.as_string().unwrap() != "granted" {
                    return;
                }
            }

            let args = to_value(&ToastArgs {
                title: "Tauri Toast Notification",
                body: &message,
            })
            .unwrap();

            sendNotification(args)
        });
    };

    view! {
        <main class="container">
            <form class="row" on:submit=toast_notification>
                <input
                    id="toast-msg-input"
                    placeholder="Enter a message..."
                    on:input=update_toast_message
                />
                <button type="submit">"Yea Toast!"</button>
            </form>
        </main>
    }
}
