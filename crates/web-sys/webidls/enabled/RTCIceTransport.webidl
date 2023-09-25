enum RTCIceTransportState {
    "new",
    "checking",
    "connected",
    "completed",
    "disconnected",
    "failed"
};

interface RTCIceTransport {
    readonly attribute RTCIceTransportState state;
};