//! BGP error code module that maintains explicit error codes assigned by IANA.
//!
//! The full list of IANA error code assignments for BGP can be viewed at here:
//! <https://www.iana.org/assignments/bgp-parameters/bgp-parameters.xhtml#bgp-parameters-3>.
use serde::Serialize;

/// BGP Error Code
///
/// The BGP error codes for notification messages:
/// <https://www.iana.org/assignments/bgp-parameters/bgp-parameters.xhtml#bgp-parameters-3>
///
/// The codes are defined in [RFC4271](https://www.iana.org/go/rfc4271) and [RFC7313](https://www.iana.org/go/rfc7313).
#[allow(non_camel_case_types)]
#[derive(Debug, Primitive, PartialEq, Eq, Hash, Copy, Clone, Serialize)]
pub enum BgpErrorCode {
    /// 0 -- reserved
    RESERVED = 0,

    /// 1 -- message header error
    MESSAGE_HEADER_ERROR = 1,

    /// 2 -- open message error
    OPEN_MESSAGE_ERROR = 2,

    /// 3 -- update message error
    UPDATE_MESSAGE_ERROR = 3,

    /// 4 -- hold timer expired
    HOLD_TIMER_EXPIRED = 4,

    /// 5 -- BGP finite state machine error
    FINITE_STATE_MACHINE_ERROR = 5,

    /// 6 -- BGP Cease notification
    CEASE = 6,

    /// 7 -- Route-Refresh message error
    ROUTE_REFRESH_MESSAGE_ERROR = 7,
    // 8 - 255: unassigned
}

/// BGP Error Subcode enum.
///
/// <https://www.iana.org/assignments/bgp-parameters/bgp-parameters.xhtml#bgp-parameters-4>
#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone, Serialize)]
pub enum BgpErrorSubcode {
    MessageHeaderError(MessageHeaderErrorSubcode),
    OpenMessageError(OpenMessageErrorSubcode),
    UpdateMessageError(UpdateMessageErrorSubcode),
    HoldTimerExpired,
    BgpFiniteStateMachineError(BgpFiniteStateMachineErrorSubcode),
    BgpCeaseNotification(BgpCeaseNotificationMessageSubcode),
    BgpRouteFreshMessageError(BgpRouteRefreshMessageErrorSubcode),
}

/// Message Header Error subcodes
///
/// <https://www.iana.org/assignments/bgp-parameters/bgp-parameters.xhtml#bgp-parameters-5>
///
/// *See source code for number assignment*
#[allow(non_camel_case_types)]
#[derive(Debug, Primitive, PartialEq, Eq, Hash, Copy, Clone, Serialize)]
pub enum MessageHeaderErrorSubcode {
    UNSPECIFIED = 0,
    CONNECTION_NOT_SYNCHRONIZED = 1,
    BAD_MESSAGE_LENGTH = 2,
    BAD_MESSAGE_TYPE = 3,
    // 4 - 255: unassigned
}


/// OPEN Message Error subcodes
///
/// <https://www.iana.org/assignments/bgp-parameters/bgp-parameters.xhtml#bgp-parameters-6>
///
/// *See source code for number assignment*
#[allow(non_camel_case_types)]
#[derive(Debug, Primitive, PartialEq, Eq, Hash, Copy, Clone, Serialize)]
pub enum OpenMessageErrorSubcode {
    UNSPECIFIED = 0,
    UNSUPPORTED_VERSION_NUMBER = 1,
    BAD_PEER_AS = 2,
    BAD_BGP_IDENTIFIER = 3,
    UNSUPPORTED_OPTIONAL_PARAMETER = 4,
    // 5 -- deprecated
    UNACCEPTABLE_HOLD_TIME = 6,
    UNSUPPORTED_CAPACITY = 7,
    // 8 -- deprecated
    // 9 -- deprecated
    // 10 -- deprecated
    ROLE_MISMATCH = 11,
    // 12 - 255: unassinged
}

/// UPDATE Message Error subcodes
///
/// <https://www.iana.org/assignments/bgp-parameters/bgp-parameters.xhtml#bgp-finite-state-machine-error-subcodes>
///
/// *See source code for number assignment*
#[allow(non_camel_case_types)]
#[derive(Debug, Primitive, PartialEq, Eq, Hash, Copy, Clone, Serialize)]
pub enum UpdateMessageErrorSubcode {
    UNSPECIFIED = 0,
    MALFORMED_ATTRIBUTE_LIST = 1,
    UNRECOGNIZED_WELL_KNOWN_ATTRIBUTE = 2,
    MISSING_WELL_KNOWN_ATTRIBUTE = 3,
    ATTRIBUTE_FLAGS_ERROR = 4,
    ATTRIBUTE_LENGTH_ERROR = 5,
    INVALID_ORIGIN_ERROR = 6,
    // 7 - deprecated
    INVALID_NEXT_HOP_ATTRIBUTE = 8,
    OPTIONAL_ATTRIBUTE_ERROR = 9,
    INVALID_NETWORK_FIELD = 10,
    MALFORMED_AS_PATH = 11,
    // 12 - 255: unassigned
}

/// BGP Finite State Machine Error Subcodes
///
/// <https://www.iana.org/assignments/bgp-parameters/bgp-parameters.xhtml#bgp-finite-state-machine-error-subcodes>
///
/// *See source code for number assignment*
#[allow(non_camel_case_types)]
#[derive(Debug, Primitive, PartialEq, Eq, Hash, Copy, Clone, Serialize)]
pub enum BgpFiniteStateMachineErrorSubcode {
    UNSPECIFIED = 0,
    RECEIVE_UNEXPECTED_MESSAGE_IN_OPENSENT_State = 1,
    RECEIVE_UNEXPECTED_MESSAGE_IN_OPENCONFIRM_STATE = 2,
    RECEIVE_UNEXPECTED_MESSAGE_IN_ESTABLISHED_STATE = 3,
    // 4 - 255: unassigned
}


/// BGP Cease NOTIFICATION message subcodes
///
/// <https://www.iana.org/assignments/bgp-parameters/bgp-parameters.xhtml#bgp-parameters-8>
///
/// *See source code for number assignment*
#[allow(non_camel_case_types)]
#[derive(Debug, Primitive, PartialEq, Eq, Hash, Copy, Clone, Serialize)]
pub enum BgpCeaseNotificationMessageSubcode {
    RESERVED = 0,
    MAXIMUM_NUMBER_OF_PREFIXES_REACHED = 1,
    ADMINISTRATIVE_SHUTDOWN = 2,
    PEER_DE_CONFIGURED = 3,
    ADMINISTRATIVE_RESET = 4,
    CONNECTION_REJECTED = 5,
    OTHER_CONFIGURATION_CHANGE = 6,
    CONNECTION_COLLISION_RESOLUTION = 7,
    OUT_OF_RESOURCES = 8,
    HARD_RESET = 9,
    BFD_DOWN = 10, // TEMPORARY - registered 2022-02-23, expires 2023-02-23
    // 11 - 255: unassigned
}

/// BGP ROUTE-REFRESH Message Error subcodes
///
/// <https://www.iana.org/assignments/bgp-parameters/bgp-parameters.xhtml#route-refresh-error-subcodes>
///
/// *See source code for number assignment*
#[allow(non_camel_case_types)]
#[derive(Debug, Primitive, PartialEq, Eq, Hash, Copy, Clone, Serialize)]
pub enum BgpRouteRefreshMessageErrorSubcode {
    RESERVED = 0,
    INVALID_MESSAGE_LENGTH = 1,
    // 2 - 255: unassigned
}
