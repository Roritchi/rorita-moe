use crate::components::{canvas::Spider, counter_btn::Button, steam_pp::SteamProfilePicture};
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

            <header>
                <SteamProfilePicture />
                <div class="card-container">
                    <div class="card-hover">
                        <div class="card">
                            <div class="card-content">
                                <div
                                    class="image"
                                    style="background-image: url(/images/anime-girl-rooftop-cars.png);"
                                ></div>
                            </div>
                        </div>
                    </div>
                    <div class="card-hover">
                        <div class="card">
                            <div class="card-content">
                                <div
                                    class="image"
                                    style="background-image: url(https://image.ibb.co/f1uHWR/cait.jpg);"
                                ></div>
                            </div>
                        </div>
                    </div>
                    <div class="card-hover">
                        <div class="card">
                            <div class="card-content">
                                <div
                                    class="image"
                                    style="background-image: url(https://image.ibb.co/kakrrR/vayne.jpg);"
                                ></div>
                            </div>
                        </div>
                    </div>
                    <div class="card-hover">
                        <div class="card">
                            <div class="card-content">
                                <div
                                    class="image"
                                    style="background-image: url(https://image.ibb.co/gHJcWR/draven.jpg);"
                                ></div>
                            </div>
                        </div>
                    </div>
                </div>
            </header>

            <div class="container">

                <Spider />

                <h1>"Rorita"</h1>

                <img src="/images/trace.moe.webp" alt="Trace Moe Logo" height="200" width="200" />
            </div>
        </ErrorBoundary>
    }
}
