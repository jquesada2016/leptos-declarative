use leptos::*;
use leptos_dom::Transparent;
use std::cell::Cell;

api_planning! {
  view! { cx,
    <If signal=bool_signal>
      <Then>
        "thing to show if bool_signal is true"
      </Then>
      <ElseIf signal=bool_signal_b>
        "Other thing to show"
      </ElseIf>
      <Else>
        "The fallback"
      </Else>
    </If>
  }
}

/// The `if` construct in component form.
///
/// [`Then`] is the only required child component, as it's what will be shown
/// when the [`If`]'s signal is true.
///
/// For more docs on allowed child components, check out [`IfProps::children`].
///
/// # Examples
///
/// ### Simple `if`
/// ```rust
/// use leptos::*;
/// use leptos_declarative::*;
///
/// # let _ = create_scope(create_runtime(), |cx| {
/// let (a, _) = create_signal(cx, true);
///
/// view! { cx,
/// <If signal=a>
///   <Then>"a is true!"</Then>
/// </If>
/// };
/// # });
/// ```
///
/// ### `if/else`
/// ```rust
/// use leptos::*;
/// use leptos_declarative::*;
///
/// # let _ = create_scope(create_runtime(), |cx| {
/// let (a, _) = create_signal(cx, true);
///
/// view! { cx,
/// <If signal=a>
///   <Then>"A is true!"</Then>
///   <Else>"A is false!"</Else>
/// </If>
/// };
/// # });
/// ```
///
/// ### `if/else-if`
/// ```rust
/// use leptos::*;
/// use leptos_declarative::*;
///
/// # let _ = create_scope(create_runtime(), |cx| {
/// let (a, _) = create_signal(cx, true);
/// let (b, _) = create_signal(cx, false);
///
/// view! { cx,
/// <If signal=a>
///   <Then>"A is true!"</Then>
///   <ElseIf signal=b>"B is true!"</ElseIf>
///   <Else>"Both A and B are false!"</Else>
/// </If>
/// };
/// # });
/// ```
#[component]
pub fn If<S>(
  cx: Scope,
  /// The bool signal.
  signal: S,
  /// The `if` conditions you would like to evaluate.
  ///
  /// Children must be any
  /// - [`Then`]
  /// - [`ElseIf`]
  /// - [`Else`]
  ///
  /// Any other child not in the above list will not be rendered.
  ///
  /// [`Then`] must be present and the first child.
  ///
  /// [`Else`] must be the last child.
  children: Box<dyn Fn(Scope) -> Fragment>,
) -> impl IntoView
where
  S: Fn() -> bool + 'static,
{
  // Memoize the signal
  let signal = create_memo(cx, move |_| signal());

  let children = children(cx);

  // Get the condition blocks
  let if_blocks = children
    .as_children()
    .iter()
    .filter_map(View::as_transparent)
    .cloned()
    .collect::<Vec<_>>();

  #[cfg(debug_assertions)]
  run_debug_checks(&if_blocks);

  let last_rendered_block = Cell::<Option<usize>>::new(None);
  let child = Cell::new(().into_view(cx));

  move || {
    let mut if_blocks = if_blocks
      .iter()
      .filter_map(Transparent::downcast_ref::<IfBlock>)
      .enumerate();

    // Subscribe all <ElseIf /> blocks
    if_blocks.clone().skip(1).for_each(|(_, block)| {
      if let IfBlock::ElseIf { signal, .. } = block {
        signal.with(|_| {});
      }
    });

    if signal() {
      if last_rendered_block.get() != Some(0) {
        last_rendered_block.set(Some(0));

        let new_child = if_blocks.next().unwrap().1.render(cx).into_view(cx);

        child.set(new_child);
      }
    } else if let Some((i, block)) =
      if_blocks.find(|(_, block)| block.is_true())
    {
      if last_rendered_block.get() != Some(i) {
        last_rendered_block.set(Some(i));

        let new_child = block.render(cx).into_view(cx);

        child.set(new_child);
      }
    } else {
      last_rendered_block.set(None);

      child.set(().into_view(cx));
    }

    let view = child.take();
    child.set(view.clone());

    view
  }
}

/// This must be the first direct child of [`If`]. It will be shown
/// iff the signal provided to [`If`] is true.
#[component(transparent)]
pub fn Then(
  cx: Scope,
  /// What you want to show when this `if` expression is evaluated.
  children: Box<dyn Fn(Scope) -> Fragment>,
) -> impl IntoView {
  let _ = cx;

  IfBlock::If { children }
}

/// This must be the direct child of an [`If`] component, and be placed after
/// the [`Then`] component. It will render it's children iff the [`If`] signal
/// is false and all other [`ElseIf`] signals are false and this one is true.
#[component(transparent)]
pub fn ElseIf<S>(
  cx: Scope,
  /// The bool signal.
  signal: S,
  /// What you want to show when this `else if` expression is evaluated.
  children: Box<dyn Fn(Scope) -> Fragment>,
) -> impl IntoView
where
  S: Fn() -> bool + 'static,
{
  let signal = create_memo(cx, move |_| signal());

  IfBlock::ElseIf { signal, children }
}

/// This must be the direct child of an [`If`] component, and be the last component.
/// It will render it's children iff all other signals are false.
#[component(transparent)]
pub fn Else(
  cx: Scope,
  /// What you want to show when all other signals are false.
  children: Box<dyn Fn(Scope) -> Fragment>,
) -> impl IntoView {
  let _ = cx;

  IfBlock::Else { children }
}

/// Represents an if block which is returned by [`Then`], [`ElseIf`]
/// or [`Else`] components.
pub enum IfBlock {
  /// The initial `if` condition, returned by [`Then`].
  If {
    /// The children method.
    children: Box<dyn Fn(Scope) -> Fragment>,
  },
  /// An `else if` condition, returned by [`ElseIf`].
  ElseIf {
    /// The signal which must evaluate to true to be rendered.
    signal: Memo<bool>,
    /// The children method.
    children: Box<dyn Fn(Scope) -> Fragment>,
  },
  /// The `else` condition, returned by [`Else`].
  Else {
    /// The children method.
    children: Box<dyn Fn(Scope) -> Fragment>,
  },
}

impl IfBlock {
  fn is_true(&self) -> bool {
    if let Self::ElseIf { signal, .. } = self {
      signal()
    } else {
      self.is_else()
    }
  }

  fn is_if(&self) -> bool {
    matches!(self, Self::If { .. })
  }

  fn is_else(&self) -> bool {
    matches!(self, Self::Else { .. })
  }

  fn render(&self, cx: Scope) -> Fragment {
    match self {
      Self::If { children } => children(cx),
      Self::ElseIf { children, .. } => children(cx),
      Self::Else { children } => children(cx),
    }
  }
}

impl IntoView for IfBlock {
  fn into_view(self, _: Scope) -> View {
    View::Transparent(Transparent::new(self))
  }
}

#[cfg(debug_assertions)]
fn run_debug_checks(if_blocks: &[Transparent]) {
  let if_blocks = if_blocks
    .iter()
    .filter_map(Transparent::downcast_ref::<IfBlock>);

  // Make sure <Show /> is first
  assert!(
    if_blocks.clone().next().unwrap().is_if(),
    "`<Show />` must be the first child of `<If />`"
  );

  // Make sure there is no more than 1 <Show />
  assert_eq!(
    if_blocks.clone().filter(|block| block.is_if()).count(),
    1,
    "there must not be more than 1 `<Show />` children within `<If />`"
  );

  // Make sure <Else /> is last
  if let Some(pos) = if_blocks.clone().position(|block| block.is_else()) {
    assert_eq!(
      pos,
      if_blocks.clone().count() - 1,
      "`<Else />` must be the last child of `<If />`"
    );
  }

  // Make sure there is no more than 1 <Else />
  assert!(
    if_blocks.filter(|block| block.is_else()).count() <= 1,
    "there must not be more than 1 `<Else />` children within `<If />`"
  );
}
