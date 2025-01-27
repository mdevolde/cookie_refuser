use crate::button_data::ButtonData;
use serde_json::json;
use wasm_bindgen::JsValue;

/// Sends the button data to the a local server on `http://localhost:5000/add`.
///
/// # Parameters
/// - `data`: A reference to a `ButtonData` instance containing the data of the button element.
///
/// # Returns
/// - `Ok(())` if the data is successfully sent to the server.
/// - `Err(JsValue)` if any error occurs during the request.
///
/// # Example
/// ```no_run
/// let button_data = ButtonData {
///     class: "button".to_string(),
///     aria_label: "Button".to_string(),
///     role: "button".to_string(),
///     text: "Click me".to_string(),
///     parent_class: "parent".to_string(),
///     position: 0,
/// };
/// send_to_server(&button_data);
/// ```
pub async fn send_to_server(data: &ButtonData) -> Result<(), JsValue> {
    let url = "http://127.0.0.1:5000/add";
    let client = reqwest::Client::new();

    let body = json!(data);

    let response = client
        .post(url)
        .json(&body)
        .send()
        .await
        .map_err(|e| JsValue::from_str(&format!("Sending error : {}", e)))?;

    if response.status().is_success() {
        Ok(())
    } else {
        Err(JsValue::from_str("Error during sending to server"))
    }
}

/// Sends the clicked button data to the a local server on `http://localhost:5000/user_click`.
///
/// # Parameters
/// - `data`: A reference to a `ButtonData` instance containing the data of the button element.
///
/// # Returns
/// - `Ok(())` if the data is successfully sent to the server.
/// - `Err(JsValue)` if any error occurs during the request.
///
/// # Example
/// ```no_run
/// let button_data = ButtonData {
///     class: "button".to_string(),
///     aria_label: "Button".to_string(),
///     role: "button".to_string(),
///     text: "Click me".to_string(),
///     parent_class: "parent".to_string(),
///     position: 0,
/// };
/// send_clicked_button_to_server(&button_data);
/// ```
pub async fn send_clicked_button_to_server(data: &ButtonData) -> Result<(), JsValue> {
    let url = "http://127.0.0.1:5000/user_click";
    let client = reqwest::Client::new();

    let body = json!(data);

    let response = client
        .post(url)
        .json(&body)
        .send()
        .await
        .map_err(|e| JsValue::from_str(&format!("Sending error : {}", e)))?;

    if response.status().is_success() {
        Ok(())
    } else {
        Err(JsValue::from_str("Error during sending to server"))
    }
}
