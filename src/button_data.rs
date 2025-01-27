use serde::{Deserialize, Serialize};
use wasm_bindgen::JsCast;
use web_sys::HtmlElement;

/// A struct representing the data of a button element.
/// This struct is used to serialize the data of a button element to JSON.
#[derive(Serialize, Deserialize, Clone)]
pub struct ButtonData {
    /// The class attribute of the button element.
    pub class: String,
    /// The aria-label attribute of the button element.
    pub aria_label: String,
    /// The role attribute of the button element.
    pub role: String,
    /// The text content of the button element.
    pub text: String,
    /// The class attribute of the parent element of the button element.
    pub parent_class: String,
    /// The position of the button element within its parent element.
    pub position: usize,
}

impl ButtonData {
    /// Creates a new `ButtonData` instance from an `HtmlElement`.
    ///
    /// # Parameters
    /// - `element`: An `HtmlElement` representing the button element.
    ///
    /// # Returns
    /// - A `ButtonData` instance containing the data of the button element.
    ///
    /// # Example
    /// ```rust
    /// let button_element = document.get_element_by_id("button").unwrap();
    /// let button_data = ButtonData::from_element(&button_element);
    /// ```
    pub fn from_element(element: &HtmlElement) -> ButtonData {
        let class = element.get_attribute("class").unwrap_or_default();
        let aria_label = element.get_attribute("aria-label").unwrap_or_default();
        let role = element.get_attribute("role").unwrap_or_default();
        let text = element.text_content().unwrap_or_default();
        let parent_class = element
            .parent_element()
            .and_then(|parent| parent.get_attribute("class"))
            .unwrap_or_default();
        let position = element
            .parent_element()
            .and_then(|parent| {
                let children = parent.children();
                for i in 0..children.length() {
                    if let Some(child) = children.item(i) {
                        if child == *element.dyn_ref::<web_sys::Element>().unwrap() {
                            return Some(i);
                        }
                    }
                }
                None
            })
            .unwrap_or(0);

        ButtonData {
            class,
            aria_label,
            role,
            text,
            parent_class,
            position: position as usize,
        }
    }
}
