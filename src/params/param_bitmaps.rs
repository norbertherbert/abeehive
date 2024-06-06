use super::param_comp_type_props::ParamBitmapBit;

pub struct TransmitStratCustomBit;
impl TransmitStratCustomBit {
    pub const NO_ADR_IN_STATIC: ParamBitmapBit = ParamBitmapBit {
        bit_number: 0,
        description: "No ADR in static mode",
    };
    pub const DOUBLE_TRANSMISSION: ParamBitmapBit = ParamBitmapBit {
        bit_number: 1,
        description: "Double transmission enabled",
    };

    pub const DR_DIST_1ST_TRANSMISSION_BIT0: ParamBitmapBit = ParamBitmapBit {
        bit_number: 2,
        description: "1st tr: data rate distribution (Bit 0)",
    };
    pub const DR_DIST_1ST_TRANSMISSION_BIT1: ParamBitmapBit = ParamBitmapBit {
        bit_number: 3,
        description: "1st tr: data rate distribution (Bit 1)",
    };
    pub const DR_DIST_1ST_TRANSMISSION_BIT2: ParamBitmapBit = ParamBitmapBit {
        bit_number: 4,
        description: "1st tr: data rate distribution (Bit 2)",
    };

    pub const DR_DIST_2ND_TRANSMISSION_BIT0: ParamBitmapBit = ParamBitmapBit {
        bit_number: 5,
        description: "2nd tr: data rate distribution (Bit 0)",
    };
    pub const DR_DIST_2ND_TRANSMISSION_BIT1: ParamBitmapBit = ParamBitmapBit {
        bit_number: 6,
        description: "2nd tr: data rate distribution (Bit 1)",
    };
    pub const DR_DIST_2ND_TRANSMISSION_BIT2: ParamBitmapBit = ParamBitmapBit {
        bit_number: 7,
        description: "2nd tr: data rate distribution (Bit 2)",
    };

    pub const FIRST_TRANSM_DR0_ENABLED: ParamBitmapBit = ParamBitmapBit {
        bit_number: 8,
        description: "TX1 DR0/SF12-125kHz",
    };
    pub const FIRST_TRANSM_DR1_ENABLED: ParamBitmapBit = ParamBitmapBit {
        bit_number: 9,
        description: "TX1 DR1/SF11-125kHz",
    };
    pub const FIRST_TRANSM_DR2_ENABLED: ParamBitmapBit = ParamBitmapBit {
        bit_number: 10,
        description: "TX1 DR2/SF10-125kHz",
    };
    pub const FIRST_TRANSM_DR3_ENABLED: ParamBitmapBit = ParamBitmapBit {
        bit_number: 11,
        description: "TX1 DR3/SF9-125kHz",
    };
    pub const FIRST_TRANSM_DR4_ENABLED: ParamBitmapBit = ParamBitmapBit {
        bit_number: 12,
        description: "TX1 DR4/SF8-125kHz",
    };
    pub const FIRST_TRANSM_DR5_ENABLED: ParamBitmapBit = ParamBitmapBit {
        bit_number: 13,
        description: "TX1 DR5/SF7-125kHz",
    };
    pub const FIRST_TRANSM_DR6_ENABLED: ParamBitmapBit = ParamBitmapBit {
        bit_number: 14,
        description: "TX1 DR6/SF7-250kHz",
    };
    pub const FIRST_TRANSM_DR7_ENABLED: ParamBitmapBit = ParamBitmapBit {
        bit_number: 15,
        description: "TX1 DR7/FSK-50kbps",
    };

    pub const SECOND_TRANSM_DR0_ENABLED: ParamBitmapBit = ParamBitmapBit {
        bit_number: 16,
        description: "TX2 DR0/SF12-125kHz",
    };
    pub const SECOND_TRANSM_DR1_ENABLED: ParamBitmapBit = ParamBitmapBit {
        bit_number: 17,
        description: "TX2 DR1/SF11-125kHz",
    };
    pub const SECOND_TRANSM_DR2_ENABLED: ParamBitmapBit = ParamBitmapBit {
        bit_number: 18,
        description: "TX2 DR2/SF10-125kHz",
    };
    pub const SECOND_TRANSM_DR3_ENABLED: ParamBitmapBit = ParamBitmapBit {
        bit_number: 19,
        description: "TX2 DR3/SF9-125kHz",
    };
    pub const SECOND_TRANSM_DR4_ENABLED: ParamBitmapBit = ParamBitmapBit {
        bit_number: 20,
        description: "TX2 DR4/SF8-125kHz",
    };
    pub const SECOND_TRANSM_DR5_ENABLED: ParamBitmapBit = ParamBitmapBit {
        bit_number: 21,
        description: "TX2 DR5/SF7-125kHz",
    };
    pub const SECOND_TRANSM_DR6_ENABLED: ParamBitmapBit = ParamBitmapBit {
        bit_number: 22,
        description: "TX2 DR6/SF7-250kHz",
    };
    pub const SECOND_TRANSM_DR7_ENABLED: ParamBitmapBit = ParamBitmapBit {
        bit_number: 23,
        description: "TX2 DR7/FSK-50kbps",
    };

    pub const VARIANTS: &'static [ParamBitmapBit] = &[
        Self::NO_ADR_IN_STATIC,
        Self::DOUBLE_TRANSMISSION,
        Self::DR_DIST_1ST_TRANSMISSION_BIT0,
        Self::DR_DIST_1ST_TRANSMISSION_BIT1,
        Self::DR_DIST_1ST_TRANSMISSION_BIT2,
        Self::DR_DIST_2ND_TRANSMISSION_BIT0,
        Self::DR_DIST_2ND_TRANSMISSION_BIT1,
        Self::DR_DIST_2ND_TRANSMISSION_BIT2,
        Self::FIRST_TRANSM_DR0_ENABLED,
        Self::FIRST_TRANSM_DR1_ENABLED,
        Self::FIRST_TRANSM_DR2_ENABLED,
        Self::FIRST_TRANSM_DR3_ENABLED,
        Self::FIRST_TRANSM_DR4_ENABLED,
        Self::FIRST_TRANSM_DR5_ENABLED,
        Self::FIRST_TRANSM_DR6_ENABLED,
        Self::FIRST_TRANSM_DR7_ENABLED,
        Self::SECOND_TRANSM_DR0_ENABLED,
        Self::SECOND_TRANSM_DR1_ENABLED,
        Self::SECOND_TRANSM_DR2_ENABLED,
        Self::SECOND_TRANSM_DR3_ENABLED,
        Self::SECOND_TRANSM_DR4_ENABLED,
        Self::SECOND_TRANSM_DR5_ENABLED,
        Self::SECOND_TRANSM_DR6_ENABLED,
        Self::SECOND_TRANSM_DR7_ENABLED,
    ];
}

pub struct ConfigFlagsBit;
impl ConfigFlagsBit {
    pub const FRAME_PENDING_ENABLED: ParamBitmapBit = ParamBitmapBit {
        bit_number: 0,
        description: "Frame pending mechanism is enabled",
    };
    pub const LONG_BUTTON_PRESS_FOR_OFF: ParamBitmapBit = ParamBitmapBit {
        bit_number: 1,
        description: "Long button press activates OFF mode",
    };
    // pub const DEPRECATED: ConfParamBitmapBit = ConfParamBitmapBit{
    //     value: 2, text: "Deprecated"
    // };
    pub const CONF_DL_ACK: ParamBitmapBit = ParamBitmapBit {
        bit_number: 3,
        description: "Config downlinks are acknowledged",
    };
    pub const WIFI_SCAN_ENC: ParamBitmapBit = ParamBitmapBit {
        bit_number: 4,
        description: "WiFi scan payload is encrypted",
    };
    pub const BLE_ACTIVE_AT_START: ParamBitmapBit = ParamBitmapBit {
        bit_number: 5,
        description: "BLE connectivity interface is active at start",
    };
    pub const WIFI_SCAN_ON_GEOLOC_START: ParamBitmapBit = ParamBitmapBit {
        bit_number: 6,
        description: "Wi-Fi scan when the GNSS geoloc procdedure starts",
    };
    pub const LED_BLINKS_ON_GPS_FIX: ParamBitmapBit = ParamBitmapBit {
        bit_number: 7,
        description: "LED blinks when a GPS fix is received",
    };
    pub const MOTION_START_EVENT_ENABLED: ParamBitmapBit = ParamBitmapBit {
        bit_number: 8,
        description: "Motion Start event messages are enabled",
    };
    pub const MOTION_END_EVENT_ENABLED: ParamBitmapBit = ParamBitmapBit {
        bit_number: 9,
        description: "Motion End event messages are enabled",
    };
    pub const JOIN_REQ_ON_LEAVING_OFF: ParamBitmapBit = ParamBitmapBit {
        bit_number: 10,
        description: "A new Join Request is sent upon leaving OFF mode",
    };
    pub const ASYMM_BLE_PAIRING_REJECTED: ParamBitmapBit = ParamBitmapBit {
        bit_number: 11,
        description: "Asymmetric BLE pairing is rejected",
    };
    pub const LONG_WIFI_PAYLOAD: ParamBitmapBit = ParamBitmapBit {
        bit_number: 12,
        description: "Long Wi-Fi payload with up to 12 BSSID is enabled",
    };
    pub const COLLECTION_OF_LONG_REPORT: ParamBitmapBit = ParamBitmapBit {
        bit_number: 13,
        description: "Collection of Long Report enabled",
    };
    pub const AUTOSTART_ON_LEAVING_SHIPPING_STATE: ParamBitmapBit = ParamBitmapBit {
        bit_number: 14,
        description: "Autostart of the tracker when leaving shipping state",
    };
    pub const OFF_MODE_FORBIDDEN: ParamBitmapBit = ParamBitmapBit {
        bit_number: 15,
        description: "OFF mode is forbidden",
    };
    pub const BEEPS_IN_SOS_MODE: ParamBitmapBit = ParamBitmapBit {
        bit_number: 16,
        description: "Beeps are played during SOS mode",
    };
    pub const ADR_ON_TOO_LONG_PAYLOAD: ParamBitmapBit = ParamBitmapBit {
        bit_number: 17,
        description: "ADR is enabled in case of too long payloads",
    };
    pub const EXTENDED_POSITION_PAYLOAD: ParamBitmapBit = ParamBitmapBit {
        bit_number: 18,
        description: "Extended position payload enabled",
    };
    // pub const RESERVED: ConfParamBitmapBit = ConfParamBitmapBit{
    //     bit_number: 19, text: "Reserved"
    // };
    pub const CLI_OVER_BLE_ENABLED: ParamBitmapBit = ParamBitmapBit {
        bit_number: 20,
        description: "CLI over BLE is enabled",
    };
    pub const BLE_PASSKEY_ENABLED: ParamBitmapBit = ParamBitmapBit {
        bit_number: 21,
        description: "BLE passkey authentication is enabled",
    };
    pub const GNSS_FIX_AFTER_WIFI_STAN_WHEN_STATIC: ParamBitmapBit = ParamBitmapBit {
        bit_number: 22,
        description: "GNSS fix after WiFi scan when static",
    };
    pub const WIFI_BEACONING_ENABLED: ParamBitmapBit = ParamBitmapBit {
        bit_number: 23,
        description: "Wi-Fi beaconing enabled",
    };
    pub const CONTINUOUS_GNSS_OPERATION: ParamBitmapBit = ParamBitmapBit {
        bit_number: 24,
        description: "Continuous GNSS operation enabled",
    };

    pub const VARIANTS: &'static [ParamBitmapBit] = &[
        Self::FRAME_PENDING_ENABLED,
        Self::LONG_BUTTON_PRESS_FOR_OFF,
        //Self::DEPRECATED,
        Self::CONF_DL_ACK,
        Self::WIFI_SCAN_ENC,
        Self::BLE_ACTIVE_AT_START,
        Self::WIFI_SCAN_ON_GEOLOC_START,
        Self::LED_BLINKS_ON_GPS_FIX,
        Self::MOTION_START_EVENT_ENABLED,
        Self::MOTION_END_EVENT_ENABLED,
        Self::JOIN_REQ_ON_LEAVING_OFF,
        Self::ASYMM_BLE_PAIRING_REJECTED,
        Self::LONG_WIFI_PAYLOAD,
        Self::COLLECTION_OF_LONG_REPORT,
        Self::AUTOSTART_ON_LEAVING_SHIPPING_STATE,
        Self::OFF_MODE_FORBIDDEN,
        Self::BEEPS_IN_SOS_MODE,
        Self::ADR_ON_TOO_LONG_PAYLOAD,
        Self::EXTENDED_POSITION_PAYLOAD,
        // Self::RESERVED,
        Self::CLI_OVER_BLE_ENABLED,
        Self::BLE_PASSKEY_ENABLED,
        Self::GNSS_FIX_AFTER_WIFI_STAN_WHEN_STATIC,
        Self::WIFI_BEACONING_ENABLED,
        Self::CONTINUOUS_GNSS_OPERATION,
    ];
}

pub struct ConfirmedUlBitmapBit;
impl ConfirmedUlBitmapBit {
    pub const FRAME_PENDING: ParamBitmapBit = ParamBitmapBit {
        bit_number: 0x00,
        description: "Frame pending message",
    };
    pub const POSITION: ParamBitmapBit = ParamBitmapBit {
        bit_number: 0x03,
        description: "Position message",
    };
    pub const STATUS: ParamBitmapBit = ParamBitmapBit {
        bit_number: 0x04,
        description: "Power and Health status message",
    };
    pub const HEARTBEAT: ParamBitmapBit = ParamBitmapBit {
        bit_number: 0x05,
        description: "Heartbeat message",
    };
    pub const ACTIVITY_CONFIG_SHOCK_BLEMAC: ParamBitmapBit = ParamBitmapBit {
        bit_number: 0x07,
        description: "Activity/Config/Shock/BLEMAC messages",
    };
    pub const SHUTDOWN: ParamBitmapBit = ParamBitmapBit {
        bit_number: 0x09,
        description: "Shutdown message",
    };
    pub const EVENT: ParamBitmapBit = ParamBitmapBit {
        bit_number: 0x0a,
        description: "Event message",
    };
    pub const COLLECTION_SCAN: ParamBitmapBit = ParamBitmapBit {
        bit_number: 0x0b,
        description: "Collection scan message",
    };
    pub const EXTENDED_POSITION: ParamBitmapBit = ParamBitmapBit {
        bit_number: 0x0e,
        description: "Extended Position message",
    };
    pub const DEBUG: ParamBitmapBit = ParamBitmapBit {
        bit_number: 0x0f,
        description: "Debug message",
    };

    pub const VARIANTS: &'static [ParamBitmapBit] = &[
        Self::FRAME_PENDING,
        Self::POSITION,
        Self::STATUS,
        Self::HEARTBEAT,
        Self::ACTIVITY_CONFIG_SHOCK_BLEMAC,
        Self::SHUTDOWN,
        Self::EVENT,
        Self::COLLECTION_SCAN,
        Self::EXTENDED_POSITION,
        Self::DEBUG,
    ];
}
