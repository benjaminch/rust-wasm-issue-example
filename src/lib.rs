mod utils;

use wasm_bindgen::prelude::*;

use web_sys::{console};

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

/// Validate registration token form.
#[wasm_bindgen]
pub async fn validate_registration_token(backend_api_domain: String, wedding_token: String) -> Result<JsValue, JsValue> {

	console::log_1(&"backend_api_domain=".clone().into());
	console::log_1(&backend_api_domain.clone().into());
	console::log_1(&"wedding_token=".clone().into());
	console::log_1(&backend_api_domain.clone().into());

    let api_endpoint_get_guest_info = format!(
        "{backend_api_domain}/weddings/{wedding_token}/guests/registration",
        backend_api_domain = backend_api_domain.clone(),
        wedding_token = wedding_token.clone(),
    );

	console::log_1(&api_endpoint_get_guest_info.clone().into());
	
	let promise = js_sys::Promise::resolve(&true.into());
    let result = wasm_bindgen_futures::JsFuture::from(promise).await.unwrap();
    Ok(result)
}

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    Ok(())
}
