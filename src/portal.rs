//! Portals for [`leptos`].
//!
//! Portals allow rendering of elements somewhere
//! completely different than where they were declared.
//!
//! For usage examples, please refer to [`PortalInput`].

use leptos::*;
use std::any::{Any, TypeId};

api_planning! {
  struct PortalA;
  struct PortalB;

  // Somewhere up there
  view! {
    <PortalProvider>
      // rest of your app
    </PortalProvider>
  }

  // Where you want your portal to output
  view! {
    <h1>"Where'd these come from???"</h1>
    <PortalOutput id=PortalA />
    <PortalOutput id=PortalB />
  }

  // Where you want portal contents to go
  view! {
    <PortalInput id=PortalA>
      <p>"I used a portal to get here..."</p>
    </PortalInput>
    <PortalInput id=PortalB>
      <p>"The cake was really nice"</p>
      </PortalInput>
  }
}

const CONTEXT_NOT_FOUND_ERROR_MESSAGE: &str =
    "failed to find `PortalCtx`, make sure you are using `<PortalProvider />` \
   somewhere near the root of the app";

#[derive(Clone)]
struct PortalCtx(StoredValue<Vec<(TypeId, RwSignal<Option<Children>>)>>);

/// The portal provider which allows to use [`PortalInput`] and [`PortalOutput`].
///
/// This must be located somewhere near the root of your component tree, above
/// anywhere you would like to use portals.
///
/// # Examples
/// ```rust
/// use leptos::*;
/// use leptos_declarative::prelude::*;
///
/// # let runtime = create_runtime();
///
/// struct PortalId;
///
/// view! {
///   <PortalProvider>
///     <div>
///       <h1>"Portal goes here!"</h1>
///       <PortalOutput id=PortalId />
///     </div>
///
///     <PortalInput id=PortalId>
///       <p>"I went through the portal!"</p>
///     </PortalInput>
///   </PortalProvider>
/// };
/// # runtime.dispose();
/// ```
#[component]
pub fn PortalProvider(
    /// The rest of your app. [`PortalInput`] and [`PortalOutput`] can be used
    /// anywhere below this point.
    children: Children,
) -> impl IntoView {
    provide_context(PortalCtx(store_value(Default::default())));

    children()
}

/// The portal entry point. Whatever children this component has will be rendered
/// in the corresponding [`PortalOutput`] with the matching `id`, wherever in your
/// app that may be.
///
/// # Examples
/// ```rust
/// use leptos::*;
/// use leptos_declarative::prelude::*;
///
/// # let runtime = create_runtime();
///
/// struct PortalId;
///
/// view! {
///   <PortalProvider>
///     <div>
///       <h1>"Portal goes here!"</h1>
///       <PortalOutput id=PortalId />
///     </div>
///
///     <PortalInput id=PortalId>
///       <p>"I went through the portal!"</p>
///     </PortalInput>
///   </PortalProvider>
/// };
/// # runtime.dispose();
/// ```
#[component]
pub fn PortalInput<T>(
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
    let portal_ctx = use_context::<PortalCtx>().expect(CONTEXT_NOT_FOUND_ERROR_MESSAGE);

    portal_ctx.0.update_value(|portals| {
        if let Some(pos) = portals
            .iter()
            .position(|(type_id, _)| *type_id == id.type_id())
        {
            portals[pos].1.set(Some(children));
        } else {
            let children = create_rw_signal(Some(children));

            portals.push((id.type_id(), children));
        }
    });
}

/// The portal output point. Whatever children the corresponding [`Portal`} with
/// matching `id` has, will be rendered here.
///
/// # Examples
/// ```rust
/// use leptos::*;
/// use leptos_declarative::prelude::*;
///
/// # let runtime = create_runtime();
///
/// struct PortalId;
///
/// view! {
///   <PortalProvider>
///     <div>
///       <h1>"Portal goes here!"</h1>
///       <PortalOutput id=PortalId />
///     </div>
///
///     <PortalInput id=PortalId>
///       <p>"I went through the portal!"</p>
///     </PortalInput>
///   </PortalProvider>
/// };
/// # runtime.dispose();
/// ```
#[component]
pub fn PortalOutput<T>(
    /// The type used as an `id`. This must match the `id` of the
    /// corresponding [`PortalInput`].
    id: T,
) -> impl IntoView
where
    T: Any,
{
    let portal_ctx = use_context::<PortalCtx>().expect(CONTEXT_NOT_FOUND_ERROR_MESSAGE);

    let mut maybe_children_signal = None;

    portal_ctx.0.update_value(|portals| {
        let children_signal = if let Some(pos) = portals
            .iter()
            .position(|(type_id, _)| *type_id == id.type_id())
        {
            portals[pos].1
        } else {
            let children_signal = create_rw_signal(None);

            portals.push((id.type_id(), children_signal));

            children_signal
        };

        maybe_children_signal = Some(children_signal);
    });

    let children_signal = maybe_children_signal.unwrap();

    let mut children = None;
    children_signal.update(|maybe_children| children = maybe_children.take());

    children
        .map(|children| children().into_view())
        .unwrap_or_else(|| ().into_view())
}
