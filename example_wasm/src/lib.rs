shadow!(shadow);

use serde_json::json;
use shadow_rs::shadow;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[allow(clippy::const_is_empty)]
pub fn version() -> JsValue {
    let json = json!({
        "branch/tag":if shadow::BRANCH.is_empty(){ shadow::BRANCH } else{ shadow::LAST_TAG },
        "commit_hash":shadow::SHORT_COMMIT,
        "build_time":shadow::BUILD_TIME,
    });
    let val = json.to_string();
    serde_wasm_bindgen::to_value(&val).unwrap()
}
