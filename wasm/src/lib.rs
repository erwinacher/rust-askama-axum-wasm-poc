use wasm_bindgen::prelude::*;
use web_sys::{window, Document, HtmlElement};

#[wasm_bindgen]
pub fn greet() -> String {
     "Hello from WebAssembly! (Success)".to_string()
}

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
     // Attach event listener to the button
     let doc: Document = window()
          .ok_or("[error] missing `window`")?
          .document()
          .ok_or("[error] missing `document`")?;

     // Find the button element
     let btn = doc
        .get_element_by_id("action-btn")
        .expect("[error] button with id 'action-btn' missing")
        .dyn_into::<HtmlElement>()?;


     // Clone the document for use inside the closure
    let doc_clone = doc.clone();

    // Define a closure for click handler
    let closure = Closure::wrap(Box::new(move || {
        if let Some(msg) = doc_clone.get_element_by_id("msg") {
            msg.set_text_content(Some("Button clicked from WebAssembly Rust! (Success)"));
        }
    }) as Box<dyn FnMut()>);

    // Attach click listener
    btn.add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())?;
    closure.forget(); // Prevent it from being dropped

    Ok(())
}
