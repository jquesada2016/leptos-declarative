// use leptos::*;

api_planning! {
  let (any_signal, _) = create_signal(cx, "apple");

  view! { cx,
    <When signal=any_signal>
      <Is f=|signal_value| signal_value == "apple">
        "show this"
      </Is>
      <Is f=|signal_value| signal_value == "oranges">
        "show that"
      </Is>
      <Otherwise>
        "fallback"
      </Otherwise>
    </When>
  }
}

// #[component]
// fn When<T>(cx: Scope, signal: Signal<T>) -> impl IntoView
// where
//   T: 'static,
// {
//   todo!()
// }

// #[component(transparent)]
// fn Is<F, T>(cx: Scope, f: F) -> impl IntoView
// where
//   F: Fn(T) -> bool + 'static,
// {
// }
