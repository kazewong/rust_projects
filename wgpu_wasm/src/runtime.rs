use wasm_bindgen::JsCast;
use web_sys::HtmlCanvasElement;

pub fn get_canvas() -> HtmlCanvasElement {
    web_sys::window()
        .and_then(|win| win.document())
        .and_then(|doc| doc.get_element_by_id("canvas"))
        .map(|canvas| canvas.unchecked_into::<HtmlCanvasElement>())
        .expect("failed to get canvas")
}
