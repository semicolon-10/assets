use dioxus::prelude::*;
use exp::Test;
mod exp;
fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        Canvas {}
    }
}

#[component]
pub fn Canvas() -> Element {
    let js_code = r#"
    const canvas = document.getElementById('drawingCanvas');
    const ctx = canvas.getContext('2d');

    const lineLengthDisplay = document.getElementById('lineLength');

    let isDrawing = false;
    let startX = 0;
    let startY = 0;

    function getTouchPos(canvas, touchEvent) {
        const rect = canvas.getBoundingClientRect();
        const touch = touchEvent.touches[0] || touchEvent.changedTouches[0];
        return {
            x: touch.clientX - rect.left,
            y: touch.clientY - rect.top
        };
    }

    canvas.addEventListener('mousedown', (event) => {
        isDrawing = true;
        startX = event.offsetX;
        startY = event.offsetY;
    });

    canvas.addEventListener('touchstart', (event) => {
        isDrawing = true;
        const touchPos = getTouchPos(canvas, event);
        startX = touchPos.x;
        startY = touchPos.y;
        event.preventDefault(); 
    });

    canvas.addEventListener('mousemove', (event) => {
        if (isDrawing) {
            const endX = event.offsetX;
            const endY = event.offsetY;

            // Draw the line
            ctx.beginPath();
            ctx.moveTo(startX, startY);
            ctx.lineTo(endX, endY);
            ctx.stroke();

            const length = Math.sqrt(Math.pow(endX - startX, 2) + Math.pow(endY - startY, 2));

            lineLengthDisplay.textContent = length.toFixed(2);

            startX = endX;
            startY = endY;
        }
    });

    canvas.addEventListener('touchmove', (event) => {
        if (isDrawing) {
            const touchPos = getTouchPos(canvas, event);
            const endX = touchPos.x;
            const endY = touchPos.y;

            // Draw the line
            ctx.beginPath();
            ctx.moveTo(startX, startY);
            ctx.lineTo(endX, endY);
            ctx.stroke();

            const length = Math.sqrt(Math.pow(endX - startX, 2) + Math.pow(endY - startY, 2));

            lineLengthDisplay.textContent = length.toFixed(2);

            startX = endX;
            startY = endY;

            event.preventDefault(); // Prevent scrolling
        }
    });

    canvas.addEventListener('mouseup', () => {
        isDrawing = false;
    });

    canvas.addEventListener('touchend', () => {
        isDrawing = false;
    });

    canvas.addEventListener('mouseleave', () => {
        isDrawing = false;
    });

    canvas.addEventListener('touchcancel', () => {
        isDrawing = false;
    });
"#;
    use_effect(move || {
        document::eval(&js_code);
    });

    rsx! {
        div {
            h1 { "Canvas Playground" }
            p {
                span { 
                    id: "lineLength",
                    "0"
                }
                " pixels"
            }
            canvas { 
                id: "drawingCanvas",
                style: "border:1px solid black",
             }
        }
    }
}

