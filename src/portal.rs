use leptos::*;
use std::any::{
  Any,
  TypeId,
};

api_planning! {
  struct PortalA;
  struct PortalB;

  // Somewhere up there
  view! { cx,
    <PortalProvider>
      // rest of your app
    </PortalProvider>
  }

  // Where you want your portal to output
  view! { cx,
    <h1>"Where'd these come from???"</h1>
    <PortalOutput id=PortalA />
    <PortalOutput id=PortalB />
  }

  // Where you want portal contents to go
  view! { cx,
    <Portal id=PortalA>
      <p>"I used a portal to get here..."</p>
    </Portal>
    <Portal id=PortalB>
      <p>"The cake was really nice"</p>
      </Portal>
  }
}

type Children = Box<dyn Fn(Scope) -> Fragment>;

const CONTEXT_NOT_FOUND_ERROR_MESSAGE: &str =
  "failed to find `PortalCtx`, make sure you are using `<PortalProvider />` \
   somewhere near the root of the app";

#[derive(Clone)]
struct PortalCtx(StoredValue<Vec<(TypeId, RwSignal<Option<Children>>)>>);

/// The portal provider which allows to use [`Portal`] and [`PortalOutput`].
///
/// This must be located somewhere near the root of your component tree, above
/// anywhere you would like to use portals.
#[component]
pub fn PortalProvider(
  cx: Scope,
  /// The rest of your app. [`Portal`] and [`PortalOutput`] can be used
  /// anywhere below this point.
  children: Children,
) -> impl IntoView {
  provide_context(cx, PortalCtx(store_value(cx, Default::default())));

  children(cx)
}

/// The portal entry point. Whatever children this component has will be rendered
/// in the corresponding [`PortalOutput`] with the matching `id`, wherever in your
/// app that may be.
#[component]
pub fn Portal<T>(
  cx: Scope,
  /// The type used as an `id`. This must match the `id` of the
  /// corresponding [`PortalOutput`].
  id: T,
  /// The children you want to render anywhere the matching [`PortalOutput`]
  /// is located.
  children: Children,
) -> impl IntoView
where
  T: Any,
{
  let portal_ctx =
    use_context::<PortalCtx>(cx).expect(CONTEXT_NOT_FOUND_ERROR_MESSAGE);

  portal_ctx.0.update(|portals| {
    if let Some(pos) = portals
      .iter()
      .position(|(type_id, _)| *type_id == id.type_id())
    {
      portals[pos].1.set(Some(children));
    } else {
      let children = create_rw_signal(cx, Some(children));

      portals.push((id.type_id(), children));
    }
  });
}

/// The portal output point. Whatever children the corresponding [`Portal`} with
/// matching `id` has, will be rendered here.
#[component]
pub fn PortalOutput<T>(
  cx: Scope,
  /// The type used as an `id`. This must match the `id` of the
  /// corresponding [`Portal`].
  id: T,
) -> impl IntoView
where
  T: Any,
{
  let portal_ctx =
    use_context::<PortalCtx>(cx).expect(CONTEXT_NOT_FOUND_ERROR_MESSAGE);

  let mut children = None;

  portal_ctx.0.update(|portals| {
    let children_signal = if let Some(pos) = portals
      .iter()
      .position(|(type_id, _)| *type_id == id.type_id())
    {
      portals[pos].1
    } else {
      let children = create_rw_signal(cx, None);

      portals.push((id.type_id(), children));

      children
    };

    children = Some(children_signal);
  });

  let children = children.unwrap();

  move || {
    children.with(|children| {
      if let Some(children) = children {
        children(cx).into_view(cx)
      } else {
        ().into_view(cx)
      }
    })
  }
}
