use idgen::NextId;
use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment,
};

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <AutoReload options=options.clone() />
                <HydrationScripts options=options islands=true/>
                <MetaTags/>
            </head>
            <body>
                <App/>
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/aggregation_gateway.css"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>

        // content for this welcome page
        <Router>
            <main>
                <Routes fallback=|| "Page not found.".into_view()>
                    <Route path=StaticSegment("") view=HomePage/>
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    view! {
        <h1>"Welcome to Leptos!"</h1>
        <GetId/>
    }
}

#[island]
fn GetId() -> impl IntoView {
    // Creates a reactive value to update the button
    let action = ServerAction::<GetSnowId>::new();

    view! {
        <ActionForm action>
            <button>Submit</button>
            <p>The result was: {move || format!("{:?}", action.value().get())}</p>
        </ActionForm>
    }
}

#[server]
#[middleware(crate::middleware::AuthLayer)]
pub async fn get_snow_id() -> Result<i64, ServerFnError> {
    println!("2. Running server function.");
    // leptos_axum::redirect("/");
    Ok(NextId())
}
