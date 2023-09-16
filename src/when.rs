// use leptos::*;

api_planning! {
  let (any_signal, _) = create_signal("apple");

  view! {
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
// fn When<T>(signal: Signal<T>) -> impl IntoView
// where
//   T: 'static,
// {
//   todo!()
// }

// #[component(transparent)]
// fn Is<F, T>(f: F) -> impl IntoView
// where
//   F: Fn(T) -> bool + 'static,
// {
// }
