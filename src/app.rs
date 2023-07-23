use crate::error_template::{AppError, ErrorTemplate};
use leptos::tracing;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context(cx);

    view! { cx,
        <Stylesheet id="leptos" href="/pkg/start-axum.css"/>
        <Title text="Welcome to Leptos"/>
        <Router fallback=|cx| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! { cx, <ErrorTemplate outside_errors/> }
                .into_view(cx)
        }>
            <main>
                <Routes>
                    <Route
                        path=""
                        view=|cx| {
                            view! { cx, <HomePage/> }
                        }
                    />
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage(cx: Scope) -> impl IntoView {
    // Creates a reactive value to update the button

    use crate::for_test_component::ForTestComponent;
    use crate::for_test_component::Row;
    use crate::for_test_enumerate_component::ForTestEnumerateComponent;

    let rows = (0..5)
        .map(|i| Row::new(&format!("key {i}"), &format!("value {i}")))
        .collect::<Vec<_>>();

    view! { cx,
        <ForTestComponent rows=rows.clone()/>
        <hr/>
    }
}
