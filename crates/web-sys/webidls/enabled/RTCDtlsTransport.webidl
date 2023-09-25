enum RTCDtlsTransportState {
    "new",
    "connecting",
    "connected",
    "closed",
    "failed"
};

interface RTCDtlsTransport {
    readonly attribute RTCIceTransport? transport;
    readonly attribute RTCDtlsTransportState state;
};