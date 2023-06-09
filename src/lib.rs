#![deny(missing_docs)]

//! Declarative components to represent control-flow and other useful
//! constructs in the [`leptos`] web framework not directly
//! provided by default.
//!
//! This crate provides 2 main components
//!
//! - [`If`](if_::If)
//! - [`PortalInput`](portal::PortalInput)
//!
//! # Usage
//! For more usage examples, please refer to the respective
//! components' documentation, but here's a taste.
//!
//! ## If
//! ```rust
//! use leptos::*;
//! use leptos_declarative::prelude::*;
//!
//! # let _ = create_scope(create_runtime(), |cx| {
//! let (a, _) = create_signal(cx, true);
//! let (b, _) = create_signal(cx, false);
//!
//! view! { cx,
//! <If signal=a>
//!   <Then>"A is true!"</Then>
//!   <ElseIf signal=b>"B is true!"</ElseIf>
//!   <Else>"Both A and B are false!"</Else>
//! </If>
//! };
//! # });
//! ```
//!
//! ## Portal
//! ```rust
//! use leptos::*;
//! use leptos_declarative::prelude::*;
//!
//! # let _ = create_scope(create_runtime(), |cx| {
//!
//! struct PortalId;
//!
//! view! { cx,
//!   <PortalProvider>
//!     <div>
//!       <h1>"Portal goes here!"</h1>
//!       <PortalOutput id=PortalId />
//!     </div>
//!
//!     <PortalInput id=PortalId>
//!       <p>"I went through the portal!"</p>
//!     </PortalInput>
//!   </PortalProvider>
//! };
//! # });
//! ```

#[macro_use]
mod util;
pub mod if_;
pub mod portal;

/// Convenient import of all components.
pub mod prelude {
  pub use crate::{
    if_::*,
    portal::*,
  };
}
