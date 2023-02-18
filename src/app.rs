use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context(cx);

    view! {
        cx,

        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/leptos_start.css"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>

        // content for this welcome page
        <Router>
            <main>
                <Routes>
                    <Route path="" view=|cx| view! { cx, <ColorPage/> }/>
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the color page of your application.
#[component]
fn ColorPage(cx: Scope) -> impl IntoView {
    let INITIAL_COLOR: &str = "#ff0000";
    let (color, set_color) = create_signal(cx, INITIAL_COLOR.to_string());

    #[cfg(not(feature = "ssr"))]
    let starting_color = if let Ok(Some(storage)) = window().local_storage()
    {
        let start_color = storage.get_item("color").unwrap_or(Some(INITIAL_COLOR.to_string()));
        log!("starting color is {:?}", start_color);
        start_color
    } else {
        Some(INITIAL_COLOR.to_string())
    };

    #[cfg(not(feature = "ssr"))]
    if let Some(color) = starting_color {
        log!("setting with starting color to {}", color);
        set_color(color);
    }

    let change_color = move |color: String| {
        #[cfg(not(feature = "ssr"))]
        if let Ok(Some(storage)) = window().local_storage() {
            storage.set_item("color", &color).expect("Failed to set color in local storage");
        }
    
        let color_clone = color.clone();
        set_color.update(move |col| *col = color_clone);

        log!("color changed to {}", color);
    };

    create_effect(cx, move |_| {
        log!("color effect changed to {}", color.get());
    });

    let styles = move || {
        format!("color: {}", color.get())
    };

    view! { cx,
        <h1 style=styles>"Color Page"</h1>
        <input type="color" value=color on:change=move |e| change_color(event_target_value(&e)) />
    }
}
