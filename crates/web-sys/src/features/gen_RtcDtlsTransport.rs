#![allow(unused_imports)]
#![allow(clippy::all)]
use super::*;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    # [wasm_bindgen (extends = :: js_sys :: Object , js_name = RTCDtlsTransport , typescript_type = "RTCDtlsTransport")]
    #[derive(Debug, Clone, PartialEq, Eq)]
    #[doc = "The `RtcDtlsTransport` class."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCDtlsTransport)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcDtlsTransport`*"]
    pub type RtcDtlsTransport;
    #[cfg(feature = "RtcIceTransport")]
    # [wasm_bindgen (structural , method , getter , js_class = "RTCDtlsTransport" , js_name = transport)]
    #[doc = "Getter for the `transport` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCDtlsTransport/transport)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcDtlsTransport`, `RtcIceTransport`*"]
    pub fn transport(this: &RtcDtlsTransport) -> Option<RtcIceTransport>;
    #[cfg(feature = "RtcDtlsTransportState")]
    # [wasm_bindgen (structural , method , getter , js_class = "RTCDtlsTransport" , js_name = state)]
    #[doc = "Getter for the `state` field of this object."]
    #[doc = ""]
    #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/RTCDtlsTransport/state)"]
    #[doc = ""]
    #[doc = "*This API requires the following crate features to be activated: `RtcDtlsTransport`, `RtcDtlsTransportState`*"]
    pub fn state(this: &RtcDtlsTransport) -> RtcDtlsTransportState;
}
