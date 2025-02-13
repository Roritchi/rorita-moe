use crate::components::{counter_btn::Button, steam_pp::SteamProfilePicture};
use leptos::prelude::*;

/// Default Home Page
#[component]
pub fn Home() -> impl IntoView {
    view! {
        <ErrorBoundary fallback=|errors| {
            view! {
                <h1>"Uh oh! Something went wrong!"</h1>

                <p>"Errors: "</p>
                // Render a list of errors as strings - good for development purposes
                <ul>
                    {move || {
                        errors
                            .get()
                            .into_iter()
                            .map(|(_, e)| view! { <li>{e.to_string()}</li> })
                            .collect_view()
                    }}

                </ul>
            }
        }>

            <div class="container">

                <h1>"Rorita"</h1>

                <SteamProfilePicture />

                <img src="/icons/trace.moe.webp" alt="Trace Moe Logo" height="200" width="200" />

                <div class="buttons">
                    <Button />
                    <Button increment=5 />
                </div>

            </div>
        </ErrorBoundary>
    }
}
