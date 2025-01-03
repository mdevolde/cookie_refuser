use wasm_bindgen::prelude::*;
use web_sys::{window, HtmlElement, MutationObserver, MutationObserverInit};

const WORDS_TO_FIND: [&str; 10] = [
    "refuser",
    "rejeter",
    "reject",
    "decline",
    "refuse",
    "optional",
    "necessary",
    "essential",
    "options",
    "enregistrer",
];

macro_rules! click_if_contains {
    ($button:expr) => {
        WORDS_TO_FIND.iter().for_each(|word| {
            if $button
                .text_content()
                .unwrap_or_default()
                .to_lowercase()
                .contains(word)
            {
                $button.click();
            }
        });
    };
}

#[wasm_bindgen]
pub fn run() -> Result<(), JsValue> {
    if let Some(window) = window() {
        if let Some(document) = window.document() {
            if let Some(body) = document.body() {
                traverse_dom(&body);
                observe_dom_changes(&body);
            }
        }
    }

    Ok(())
}

fn traverse_dom(element: &HtmlElement) {
    let tag_name = element.tag_name().to_lowercase();
    if tag_name.contains("button") || element.get_attribute("role").unwrap_or_default() == "button"
    {
        click_if_contains!(element);
    }

    let children = element.children();
    for i in 0..children.length() {
        if let Some(child) = children.item(i) {
            if let Ok(html_element) = child.dyn_into::<HtmlElement>() {
                traverse_dom(&html_element);
            }
        }
    }

    if let Some(shadow_root) = element.shadow_root() {
        let shadow_elements = shadow_root.query_selector_all("*").unwrap();
        for i in 0..shadow_elements.length() {
            if let Some(shadow_element) = shadow_elements.item(i) {
                if let Ok(html_element) = shadow_element.dyn_into::<HtmlElement>() {
                    traverse_dom(&html_element);
                }
            }
        }
    }
}

fn observe_dom_changes(body: &HtmlElement) {
    let closure = Closure::wrap(Box::new(
        move |mutations: Vec<web_sys::MutationRecord>, _observer: MutationObserver| {
            for mutation in mutations.iter() {
                let added_nodes = mutation.added_nodes();
                for i in 0..added_nodes.length() {
                    if let Some(node) = added_nodes.item(i) {
                        if let Ok(html_element) = node.dyn_into::<HtmlElement>() {
                            traverse_dom(&html_element);
                        }
                    }
                }
            }
        },
    )
        as Box<dyn FnMut(Vec<web_sys::MutationRecord>, MutationObserver)>);

    let observer = MutationObserver::new(closure.as_ref().unchecked_ref()).unwrap();
    closure.forget();

    let config = MutationObserverInit::new();
    config.set_child_list(true);
    config.set_subtree(true);

    observer.observe_with_options(body, &config).unwrap();
}
