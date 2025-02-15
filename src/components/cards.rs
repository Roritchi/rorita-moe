use leptos::html::Style;
use leptos::logging::log;
use leptos::{ev::mousemove, html, prelude::*};
use leptos_use::use_event_listener;

use crate::components::canvas::Spider;

#[component]
pub fn Cards() -> impl IntoView {
    let card_1_ref = NodeRef::<html::Div>::new();

    let (x, set_x) = signal(50);
    let (y, set_y) = signal(100);

    // Effect::new(move |_| if let Some(card_1) = card_1_ref.get() {});

    Effect::new(move |_| {
        use_event_listener(card_1_ref, mousemove, move |evt| {
            log!(
                "click from element {:?}",
                event_target::<web_sys::HtmlDivElement>(&evt)
            );

            set_x.set(evt.page_x());
            set_y.set(evt.page_y());
        });
    });

    view! {
        <div class="card-container">
            <div
                class="card-hover"
                // on:click=move |_| {
                // if let Some(card_1) = card_1_ref.get() {
                // // card_1.class("selected");
                // }
                // }
                node_ref=card_1_ref
            >
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
        <Spider card_1_ref=card_1_ref />
    }
}
