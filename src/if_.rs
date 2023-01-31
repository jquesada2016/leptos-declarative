api_planning! {
  view! { cx,
    <If signal=bool_signal>
      <Show>
        "thing to show if bool_signal is true"
      </Show>
      <ElseIf signal=bool_signal_b>
        "Other thing to show"
      </ElseIf>
      <Else>
        "The fallback"
      </Else>
    </If>
  }
}
