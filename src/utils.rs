use crate::button_data::ButtonData;
use crate::server::send_clicked_button_to_server;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlElement;

/// Handle the click event on a button
///
/// # Arguments
/// - `target` : The target element of the event
fn handle_button_click(target: HtmlElement) {
    let tag_name = target.tag_name().to_lowercase();
    if tag_name.contains("button")
        || target.get_attribute("role").unwrap_or_default() == "button"
        || target
            .get_attribute("class")
            .unwrap_or_default()
            .contains("button")
        || target
            .get_attribute("class")
            .unwrap_or_default()
            .contains("btn")
    {
        // Récupérer les données du bouton cliqué
        let button_data = ButtonData::from_element(&target);

        web_sys::console::log_1(&JsValue::from_str(&format!(
            "Button clicked: {:?}",
            button_data.text
        )));

        spawn_local(async move {
            if let Err(e) = send_clicked_button_to_server(&button_data).await {
                web_sys::console::error_1(&e);
            }
        });
    }
}

/// Create a click handler for buttons, using the `handle_button_click` function
///
/// # Returns
/// A closure that can be used as an event listener for the click event
///
/// # Example
/// ```no_run
/// let click_handler = create_click_handler();
/// button.add_event_listener_with_callback("click", click_handler.as_ref().unchecked_ref()).unwrap();
/// click_handler.forget();
/// ```
pub fn create_click_handler() -> Closure<dyn FnMut(web_sys::Event)> {
    Closure::wrap(Box::new(move |event: web_sys::Event| {
        if let Some(target) = event.target() {
            if let Ok(html_element) = target.dyn_into::<HtmlElement>() {
                handle_button_click(html_element);
            }
        }
    }) as Box<dyn FnMut(_)>)
}
