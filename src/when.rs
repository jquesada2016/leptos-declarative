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
