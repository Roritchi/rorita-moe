use leptos::ev::mousemove;
use leptos::logging::log;
use leptos::tachys;
use leptos::wasm_bindgen::prelude::Closure;
use leptos::{html, prelude::*};
use leptos_use::{use_event_listener, use_mouse, UseMouseReturn};
use web_sys::wasm_bindgen::JsCast;
use web_sys::wasm_bindgen::JsValue;
use web_sys::{window, CanvasGradient, CanvasRenderingContext2d, HtmlCanvasElement};

#[component]
pub fn Spider(card_1_ref: NodeRef<tachys::html::element::Div>) -> impl IntoView {
    let (timeout_id, set_timeout_id) = create_signal(None);

    let canvas_ref = NodeRef::<html::Canvas>::new();

    let cancel_timeout = move || {
        if let Some(id) = timeout_id.get() {
            if let Some(win) = window() {
                win.clear_timeout_with_handle(id);
                log!("Timeout canceled!");
            }
            set_timeout_id.set(None);
        }
    };

    Effect::new(move |_| {
        if let Some(canvas) = canvas_ref.get() {
            let width = canvas.offset_width() as u32;
            let height = canvas.offset_height() as u32;

            canvas.set_width(width);
            canvas.set_height(height);

            let html_canvas: HtmlCanvasElement = Into::<HtmlCanvasElement>::into(canvas);

            let ctx = html_canvas
                .get_context("2d")
                .unwrap()
                .unwrap()
                .dyn_into::<CanvasRenderingContext2d>()
                .unwrap();

            ctx.set_fill_style(&JsValue::from_str("#A5E3E0"));

            let cctx: CanvasRenderingContext2d = ctx.clone();

            _ = use_event_listener(card_1_ref, mousemove, move |evt| {
                log!(
                    "click from element {:?}",
                    event_target::<web_sys::HtmlDivElement>(&evt)
                );

                let element = event_target::<web_sys::HtmlDivElement>(&evt);

                let x = element.get_bounding_client_rect().x()
                    + element.get_bounding_client_rect().width();

                log!("{}", x);

                cctx.begin_path();
                cctx.arc(x, 200.0, 50.0, 0.0, std::f64::consts::TAU)
                    .expect("Failed to draw arc");
                cctx.fill();

                let ccctx: CanvasRenderingContext2d = cctx.clone();

                cancel_timeout();

                if let Some(win) = window() {
                    let closure = Closure::wrap(Box::new(move || {
                        log!("Timeout finished!");
                        ccctx.clear_rect(0.into(), 0.into(), width.into(), height.into());
                    }) as Box<dyn Fn()>);

                    let id = win
                        .set_timeout_with_callback_and_timeout_and_arguments_0(
                            closure.as_ref().unchecked_ref(),
                            5000, // 5 seconds
                        )
                        .expect("Failed to set timeout");

                    closure.forget(); // Keep it alive
                    set_timeout_id.set(Some(id));
                }
            });

            // if let Some(card_1) = card_1_ref.get() {
            //     let x = card_1.client_left() + card_1.client_width();

            //     ctx.begin_path();
            //     ctx.arc(x.into(), 200.0, 50.0, 0.0, std::f64::consts::TAU)
            //         .expect("Failed to draw arc");
            //     ctx.fill();
            // }

            /*let gradient: CanvasGradient = ctx
            .create_radial_gradient(500.0, 300.0, 10.0, 500.0, 300.0, 50.0)
            .expect("Failed to create radial gradient");*/

            // Farben hinzufügen (0.0 = innen, 1.0 = außen)
            //gradient.add_color_stop(0.0, "#A5E3E0").unwrap();

            // Gradient als Füllstil setzen
            // ctx.set_fill_style(&gradient);

            // Kreis mit dem Gradient zeichnen
            ctx.begin_path();
            ctx.arc(500.0, 300.0, 50.0, 0.0, std::f64::consts::TAU)
                .expect("Failed to draw arc");
            ctx.fill();
        }
    });

    view! { <canvas node_ref=canvas_ref class="canvas" /> }
}
