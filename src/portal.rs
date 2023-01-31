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
