#![allow(unused_imports)]
#![allow(clippy::all)]
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
#[doc = "The `RtcIceTransportState` enum."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `RtcIceTransportState`*"]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RtcIceTransportState {
    New = "new",
    Checking = "checking",
    Connected = "connected",
    Completed = "completed",
    Disconnected = "disconnected",
    Failed = "failed",
}
