use leptos::{html, prelude::*};
use web_sys::wasm_bindgen::JsCast;
use web_sys::wasm_bindgen::JsValue;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

#[component]
pub fn Spider() -> impl IntoView {
    let canvas_ref = NodeRef::<html::Canvas>::new();

    Effect::new(move |_| {
        if let Some(canvas) = canvas_ref.get() {
            let width = canvas.offset_width() as u32;
            let height = canvas.offset_height() as u32;

            canvas.set_width(1920);
            canvas.set_height(1080);

            let html_canvas: web_sys::HtmlCanvasElement =
                Into::<web_sys::HtmlCanvasElement>::into(canvas);

            let ctx = html_canvas
                .get_context("2d")
                .unwrap()
                .unwrap()
                .dyn_into::<CanvasRenderingContext2d>()
                .unwrap();

            ctx.set_fill_style(&JsValue::from_str("red"));
            ctx.begin_path();
            // what ever you want to do with the canvas
            ctx.fill_rect(0f64, 0f64, 100f64, 120f64);
            ctx.fill();

            ctx.begin_path();

            // Draw the outer circle.
            ctx.arc(75.0, 75.0, 50.0, 0.0, std::f64::consts::PI * 2.0)
                .unwrap();

            // Draw the mouth.
            ctx.move_to(110.0, 75.0);
            ctx.arc(75.0, 75.0, 35.0, 0.0, std::f64::consts::PI)
                .unwrap();

            // Draw the left eye.
            ctx.move_to(65.0, 65.0);
            ctx.arc(60.0, 65.0, 5.0, 0.0, std::f64::consts::PI * 2.0)
                .unwrap();

            // Draw the right eye.
            ctx.move_to(95.0, 65.0);
            ctx.arc(90.0, 65.0, 5.0, 0.0, std::f64::consts::PI * 2.0)
                .unwrap();

            ctx.stroke();
        }
    });

    view! { <canvas node_ref=canvas_ref class="canvas" /> }
}
