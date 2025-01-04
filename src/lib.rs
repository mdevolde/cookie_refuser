use serde_wasm_bindgen::from_value;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use web_sys::{window, HtmlElement, MutationObserver, MutationObserverInit};

const LOCAL_STORAGE_KEY: &str = "cookie_refuser_click_count";

#[wasm_bindgen]
pub struct CookieRefuser {
    wordlist: Rc<Vec<String>>,
    click_counter: Rc<RefCell<usize>>,
    max_clicks: usize,
}

#[wasm_bindgen]
impl CookieRefuser {
    #[wasm_bindgen(constructor)]
    pub fn new(wordlist: JsValue, max_clicks: usize) -> Result<CookieRefuser, JsValue> {
        let vec_wordlist: Vec<String> = from_value(wordlist)?;
        let wordlist = Rc::new(vec_wordlist);

        let storage = window()
            .and_then(|w| w.local_storage().ok().flatten())
            .ok_or_else(|| JsValue::from_str("Local storage is not supported"))?;

        let initial_count = storage
            .get_item(LOCAL_STORAGE_KEY)?
            .unwrap_or_else(|| "0".to_string())
            .parse::<usize>()
            .unwrap_or(0);

        Ok(CookieRefuser {
            wordlist,
            click_counter: Rc::new(RefCell::new(initial_count)),
            max_clicks,
        })
    }

    pub fn run(&self) -> Result<(), JsValue> {
        if let Some(window) = window() {
            if let Some(document) = window.document() {
                if let Some(body) = document.body() {
                    self.traverse_dom(&body)?;
                    self.observe_dom_changes(&body)?;
                }
            }
        }
        Ok(())
    }

    fn traverse_dom(&self, element: &HtmlElement) -> Result<(), JsValue> {
        if *self.click_counter.borrow() >= self.max_clicks {
            return Ok(());
        }

        let tag_name = element.tag_name().to_lowercase();
        if tag_name.contains("button")
            || element.get_attribute("role").unwrap_or_default() == "button"
            || element
                .get_attribute("class")
                .unwrap_or_default()
                .contains("button")
        {
            self.click_if_contains(element);
        }

        let children = element.children();
        for i in 0..children.length() {
            if let Some(child) = children.item(i) {
                if let Ok(html_element) = child.dyn_into::<HtmlElement>() {
                    self.traverse_dom(&html_element)?;
                }
            }
        }

        if let Some(shadow_root) = element.shadow_root() {
            let shadow_elements = shadow_root.query_selector_all("*")?;
            for i in 0..shadow_elements.length() {
                if let Some(shadow_element) = shadow_elements.item(i) {
                    if let Ok(html_element) = shadow_element.dyn_into::<HtmlElement>() {
                        self.traverse_dom(&html_element)?;
                    }
                }
            }
        }
        Ok(())
    }

    fn observe_dom_changes(&self, body: &HtmlElement) -> Result<(), JsValue> {
        let wordlist = Rc::clone(&self.wordlist);
        let click_counter = Rc::clone(&self.click_counter);
        let max_clicks = self.max_clicks;

        let closure = Closure::wrap(Box::new(
            move |mutations: Vec<web_sys::MutationRecord>, _observer: MutationObserver| {
                for mutation in mutations.iter() {
                    let added_nodes = mutation.added_nodes();
                    for i in 0..added_nodes.length() {
                        if *click_counter.borrow() >= max_clicks {
                            return;
                        }

                        if let Some(node) = added_nodes.item(i) {
                            if let Ok(html_element) = node.dyn_into::<HtmlElement>() {
                                let _ = CookieRefuser {
                                    wordlist: Rc::clone(&wordlist),
                                    click_counter: Rc::clone(&click_counter),
                                    max_clicks,
                                }
                                .traverse_dom(&html_element);
                            }
                        }
                    }
                }
            },
        )
            as Box<dyn FnMut(Vec<web_sys::MutationRecord>, MutationObserver)>);

        let observer = MutationObserver::new(closure.as_ref().unchecked_ref())?;
        closure.forget();

        let config = MutationObserverInit::new();
        config.set_child_list(true);
        config.set_subtree(true);

        observer.observe_with_options(body, &config)?;
        Ok(())
    }

    fn click_if_contains(&self, button: &HtmlElement) {
        for word in self.wordlist.iter() {
            if button
                .text_content()
                .unwrap_or_default()
                .to_lowercase()
                .contains(word)
            {
                if *self.click_counter.borrow() < self.max_clicks {
                    button.click();
                    *self.click_counter.borrow_mut() += 1;

                    let new_count = *self.click_counter.borrow();
                    spawn_local(async move {
                        if let Some(window) = web_sys::window() {
                            if let Ok(storage) = window.local_storage() {
                                if let Some(storage) = storage {
                                    storage
                                        .set_item(LOCAL_STORAGE_KEY, &new_count.to_string())
                                        .unwrap();
                                }
                            }
                        }
                    });
                }
            }
        }
    }
}
