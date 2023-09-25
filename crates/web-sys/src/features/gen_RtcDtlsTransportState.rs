#![allow(unused_imports)]
#![allow(clippy::all)]
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
#[doc = "The `RtcDtlsTransportState` enum."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `RtcDtlsTransportState`*"]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RtcDtlsTransportState {
    New = "new",
    Connecting = "connecting",
    Connected = "connected",
    Closed = "closed",
    Failed = "failed",
}
