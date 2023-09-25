#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = RTCIceTransport , typescript_type = "RTCIceTransport")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `RtcIceTransport` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCIceTransport)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceTransport`*"]
    pub type RtcIceTransport;
    #[cfg(feature = "RtcIceTransportState")]
    # [wasm_bindgen (structural , method , getter , js_class = "RTCIceTransport" , js_name = state)]
    #[doc = "Getter for the `state` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCIceTransport/state)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcIceTransport`, `RtcIceTransportState`*"]
    pub fn state(this: &RtcIceTransport) -> RtcIceTransportState;
}
