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
                    <Route path="/read-time" view=|cx| view! { cx, <ReadTime/> }/>
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the color page of your application.
#[component]
fn ColorPage(cx: Scope) -> impl IntoView {
    const INITIAL_COLOR: &str = "#ff0000";
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

        set_color.update(|col| *col = color.clone());

        log!("color changed to {}", color);
    };

    create_effect(cx, move |_| {
        log!("color effect changed to {}", color.get());
    });

    let styles = move || {
        format!("color: {}", color())
    };

    view! { cx,
        <h1 style=styles>"Color Page"</h1>
        <input type="color" value=color on:change=move |e| change_color(event_target_value(&e)) />
    }
}

/// Renders the read time page
#[component]
fn ReadTime(cx: Scope) -> impl IntoView {
    const STANDARD_WPM: u8 = 200;

    let (text, set_text) = create_signal(cx, String::new());
    let word_count = move || text().split_whitespace().count() as u8;
    let read_time = move || {
        let time = word_count() as f32 / STANDARD_WPM as f32;
        let minutes = time.floor() as u8;
        let seconds = ((time - minutes as f32) * 60.0).round() as u8;

        if minutes == 0 && seconds == 0 {
            return "0s".to_string();
        }

        let mut minutes_str = minutes.to_string();
        let mut seconds_str = seconds.to_string();

        if minutes < 10 {
            minutes_str = format!("0{}", minutes)
        }
        if seconds < 10 {
            seconds_str = format!("0{}", seconds)
        }

        format!("{}:{}", minutes_str, seconds_str)
    };

    create_effect(cx, move |_| {
        log!("changed {}", read_time());
    });

    view! { cx,
        <h1>{read_time} " read time - Words " {word_count}</h1>
        <textarea
        rows="10"
        cols="50"
        value={text()}
        on:input=move |e| set_text(event_target_value(&e))
      ></textarea>
    }
}
