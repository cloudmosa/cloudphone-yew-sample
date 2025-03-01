use gloo::timers::callback::Timeout;
use yew::prelude::*;
use super::super::constants::TOAST_TIMEOUT_MS;

#[derive(Properties, PartialEq)]
pub struct ToastProps {
    pub message: String,
    pub on_close: Callback<()>,
}

#[function_component]
pub fn Toast(props: &ToastProps) -> Html {
    let fading = use_state(|| false);
    let on_close = props.on_close.clone();
    let fading_clone = fading.clone();

    use_effect_with(
        (),
        move |_| {
            let fading_clone = fading_clone.clone();
            let timeout = Timeout::new(TOAST_TIMEOUT_MS, move || fading_clone.set(true));
            timeout.forget();

            let close_timeout = Timeout::new(TOAST_TIMEOUT_MS, move || on_close.emit(()));
            close_timeout.forget();

            || {}
        },
    );

    html! {
        <div class={format!("toast {}", if *fading { "opacity-0" } else { "opacity-100" })}>
            { &props.message }
        </div>
    }
}
