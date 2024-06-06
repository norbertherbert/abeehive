use crate::prm::typ::{
    BitmapBit, DistinctVal, PrmDatBitmap, PrmDatDec, PrmDatDistinct, PrmDatOptional,
};

// ***********************
// *** MODE
// ***********************

pub static MODE: PrmDatDistinct = PrmDatDistinct {
    id: 0xf9,
    name: "mode",
    label: "Operation Mode",
    description: "Mode Help Text",
    default_val: ModeOption::STANDBY.val,
    distinct_vals: &[
        ModeOption::STANDBY,
        ModeOption::MOTION_TRACKING,
        ModeOption::PERMANENT_TRACKING,
        ModeOption::MOTION_START_END_TRACKING,
        ModeOption::ACTIVITY_TRACKING,
        ModeOption::OFF,
    ],
};
pub struct ModeOption;
impl ModeOption {
    pub const STANDBY: DistinctVal = DistinctVal {
        val: 0,
        txt: "Standby",
    };
    pub const MOTION_TRACKING: DistinctVal = DistinctVal {
        val: 1,
        txt: "Motion tracking",
    };
    pub const PERMANENT_TRACKING: DistinctVal = DistinctVal {
        val: 2,
        txt: "Permanent tracking",
    };
    pub const MOTION_START_END_TRACKING: DistinctVal = DistinctVal {
        val: 3,
        txt: "Motion start/end tracking",
    };
    pub const ACTIVITY_TRACKING: DistinctVal = DistinctVal {
        val: 4,
        txt: "Activity tracking",
    };
    pub const OFF: DistinctVal = DistinctVal { val: 5, txt: "Off" };
}

// ***********************
// UL_PERIOD
// ***********************

pub static UL_PERIOD: PrmDatDec = PrmDatDec {
    id: 0x00,
    name: "ul_period",
    label: "Location update period",
    description: "Location update period Help Text",
    default_val: 120,
    range: (15, 10000),
};

// ***********************
// LORA_PERIOD
// ***********************

pub static LORA_PERIOD: PrmDatDec = PrmDatDec {
    id: 0x01,
    name: "lora_period",
    label: "Heartbeat period",
    description: "Heartbeat period Help Text",
    default_val: 300,
    range: (300, 10000),
};

// ***********************
// PERIODIC_POS_PERIOD
// ***********************

pub static PERIODIC_POS_PERIOD: PrmDatOptional = PrmDatOptional {
    id: 0x03,
    name: "periodic_pos_period",
    label: "Periodic Position Report Period",
    description: "Periodic Position Report Period Help Text",
    default_val: 3600,
    disabled_val: 0,
    range: (1, 10000),
};

// ***********************
// GEOLOC_SENSOR
// ***********************

pub static GEOLOC_SENSOR: PrmDatDistinct = PrmDatDistinct {
    id: 0x05,
    name: "geoloc_sensor",
    label: "Primary Geoloc Technology",
    description: "Primary Geoloc Technology Help Text",
    default_val: GeolocSensorOption::GPS.val,
    distinct_vals: &[
        GeolocSensorOption::WIFI,
        GeolocSensorOption::GPS,
        GeolocSensorOption::LPGPS,
        // GeolocSensorOption::RESERVED_3,
        // GeolocSensorOption::RESERVED_4,
        // GeolocSensorOption::RESERVED_5,
        GeolocSensorOption::WIFI_GPS,
        GeolocSensorOption::WIFI_LPGPS,
        // GeolocSensorOption::RESERVED_8,
        GeolocSensorOption::WIFI_LPGPS_WIFI_GPS,
        GeolocSensorOption::BLE,
        GeolocSensorOption::BLE_GPS,
        GeolocSensorOption::BLE_LPGPS,
    ],
};
pub struct GeolocSensorOption;
impl GeolocSensorOption {
    pub const WIFI: DistinctVal = DistinctVal {
        val: 0,
        txt: "WIFI",
    };
    pub const GPS: DistinctVal = DistinctVal { val: 1, txt: "GPS" };
    pub const LPGPS: DistinctVal = DistinctVal {
        val: 2,
        txt: "LPGPS",
    };
    // pub const RESERVED_3: DistinctVal = DistinctVal{val: 3, txt: "Reserved_3"};
    // pub const RESERVED_4: DistinctVal = DistinctVal{val: 4, txt: "Reserved_4"};
    // pub const RESERVED_5: DistinctVal = DistinctVal{val: 5, txt: "Reserved_5"};
    pub const WIFI_GPS: DistinctVal = DistinctVal {
        val: 6,
        txt: "WIFI-GPS",
    };
    pub const WIFI_LPGPS: DistinctVal = DistinctVal {
        val: 7,
        txt: "WIFI-LPGPS",
    };
    // pub const RESERVED_8: DistinctVal = DistinctVal{val: 8, txt: "Reserved_8"};
    pub const WIFI_LPGPS_WIFI_GPS: DistinctVal = DistinctVal {
        val: 9,
        txt: "WIFI-LPGPS/WIFI-GPS",
    };
    pub const BLE: DistinctVal = DistinctVal {
        val: 10,
        txt: "BLE",
    };
    pub const BLE_GPS: DistinctVal = DistinctVal {
        val: 11,
        txt: "BLE-GPS",
    };
    pub const BLE_LPGPS: DistinctVal = DistinctVal {
        val: 12,
        txt: "BLE-LPGPS",
    };
}

// ***********************
// GEOLOC_METHOD
// ***********************

pub static GEOLOC_METHOD: PrmDatDistinct = PrmDatDistinct {
    id: 0x06,
    name: "geoloc_method",
    label: "Secondary Geoloc Technology",
    description: "Secondary Geoloc Technology Help Text",
    default_val: GeolocSensorOption::GPS.val,
    distinct_vals: &[
        GeolocMethodOption::WIFI,
        GeolocMethodOption::GPS,
        GeolocMethodOption::LPGPS,
        GeolocMethodOption::WIFI_GPS,
        GeolocMethodOption::WIFI_LPGPS,
        GeolocMethodOption::BLE,
        GeolocMethodOption::BLE_GPS,
        GeolocMethodOption::BLE_LPGPS,
    ],
};
pub struct GeolocMethodOption;
impl GeolocMethodOption {
    pub const WIFI: DistinctVal = DistinctVal {
        val: 0,
        txt: "WIFI",
    };
    pub const GPS: DistinctVal = DistinctVal { val: 1, txt: "GPS" };
    pub const LPGPS: DistinctVal = DistinctVal {
        val: 2,
        txt: "LPGPS",
    };
    pub const WIFI_GPS: DistinctVal = DistinctVal {
        val: 3,
        txt: "WIFI-GPS",
    };
    pub const WIFI_LPGPS: DistinctVal = DistinctVal {
        val: 4,
        txt: "WIFI-LPGPS",
    };
    pub const BLE: DistinctVal = DistinctVal { val: 5, txt: "BLE" };
    pub const BLE_GPS: DistinctVal = DistinctVal {
        val: 6,
        txt: "BLE-GPS",
    };
    pub const BLE_LPGPS: DistinctVal = DistinctVal {
        val: 7,
        txt: "BLE-LPGPS",
    };
}

// ***********************
// TRANSMIT_STRAT
// ***********************

pub static TRANSMIT_STRAT: PrmDatDistinct = PrmDatDistinct {
    id: 0x0e,
    name: "transmit_strat",
    label: "Transmit Strategy",
    description: "Transmit Strategy Help Text",
    default_val: 2,
    distinct_vals: &[
        TransmitStratOption::SINGLE_FIXED,
        TransmitStratOption::SINGLE_RANDOM,
        TransmitStratOption::DOUBLE_RANDOM,
        TransmitStratOption::DOUBLE_FIXED,
        // TransmitStratOption::RESERVED_4,
        TransmitStratOption::CUSTOM,
    ],
};
pub struct TransmitStratOption;
impl TransmitStratOption {
    pub const SINGLE_FIXED: DistinctVal = DistinctVal {
        val: 0,
        txt: "Single fixed",
    };
    pub const SINGLE_RANDOM: DistinctVal = DistinctVal {
        val: 1,
        txt: "Single random",
    };
    pub const DOUBLE_RANDOM: DistinctVal = DistinctVal {
        val: 2,
        txt: "Double random",
    };
    pub const DOUBLE_FIXED: DistinctVal = DistinctVal {
        val: 3,
        txt: "Doubl fixed",
    };
    // pub const RESERVED_4: DistinctVal = DistinctVal{val: 4, txt: "Reserved_4"};
    pub const CUSTOM: DistinctVal = DistinctVal {
        val: 5,
        txt: "Custom",
    };
}

// ***********************
// TRANSMIT_STRAT_CUSTOM
// ***********************

pub static TRANSMIT_STRAT_CUSTOM: PrmDatBitmap = PrmDatBitmap {
    id: 0x1e,
    name: "transmit_strat_custom",
    label: "Custom Transmit Strategy Settings",
    description: "Custom Transmit Strategy Help Text",
    default_val: 12289,
    bits: &[
        TransmitStratCustomBit::NO_ADR_IN_STATIC,
        TransmitStratCustomBit::DOUBLE_TRANSMISSION,
        TransmitStratCustomBit::DR_DIST_1ST_TRANSMISSION_BIT0,
        TransmitStratCustomBit::DR_DIST_1ST_TRANSMISSION_BIT1,
        TransmitStratCustomBit::DR_DIST_1ST_TRANSMISSION_BIT2,
        TransmitStratCustomBit::DR_DIST_2ND_TRANSMISSION_BIT0,
        TransmitStratCustomBit::DR_DIST_2ND_TRANSMISSION_BIT1,
        TransmitStratCustomBit::DR_DIST_2ND_TRANSMISSION_BIT2,
        TransmitStratCustomBit::FIRST_TRANSM_DR0_ENABLED,
        TransmitStratCustomBit::FIRST_TRANSM_DR1_ENABLED,
        TransmitStratCustomBit::FIRST_TRANSM_DR2_ENABLED,
        TransmitStratCustomBit::FIRST_TRANSM_DR3_ENABLED,
        TransmitStratCustomBit::FIRST_TRANSM_DR4_ENABLED,
        TransmitStratCustomBit::FIRST_TRANSM_DR5_ENABLED,
        TransmitStratCustomBit::FIRST_TRANSM_DR6_ENABLED,
        TransmitStratCustomBit::FIRST_TRANSM_DR7_ENABLED,
        TransmitStratCustomBit::SECOND_TRANSM_DR0_ENABLED,
        TransmitStratCustomBit::SECOND_TRANSM_DR1_ENABLED,
        TransmitStratCustomBit::SECOND_TRANSM_DR2_ENABLED,
        TransmitStratCustomBit::SECOND_TRANSM_DR3_ENABLED,
        TransmitStratCustomBit::SECOND_TRANSM_DR4_ENABLED,
        TransmitStratCustomBit::SECOND_TRANSM_DR5_ENABLED,
        TransmitStratCustomBit::SECOND_TRANSM_DR6_ENABLED,
        TransmitStratCustomBit::SECOND_TRANSM_DR7_ENABLED,
    ],
};
pub struct TransmitStratCustomBit;
impl TransmitStratCustomBit {
    pub const NO_ADR_IN_STATIC: BitmapBit = BitmapBit {
        bit: 0,
        txt: "No ADR in static mode",
    };
    pub const DOUBLE_TRANSMISSION: BitmapBit = BitmapBit {
        bit: 1,
        txt: "Double transmission enabled",
    };

    pub const DR_DIST_1ST_TRANSMISSION_BIT0: BitmapBit = BitmapBit {
        bit: 2,
        txt: "1st tr: data rate distribution (Bit 0)",
    };
    pub const DR_DIST_1ST_TRANSMISSION_BIT1: BitmapBit = BitmapBit {
        bit: 3,
        txt: "1st tr: data rate distribution (Bit 1)",
    };
    pub const DR_DIST_1ST_TRANSMISSION_BIT2: BitmapBit = BitmapBit {
        bit: 4,
        txt: "1st tr: data rate distribution (Bit 2)",
    };

    pub const DR_DIST_2ND_TRANSMISSION_BIT0: BitmapBit = BitmapBit {
        bit: 5,
        txt: "2nd tr: data rate distribution (Bit 0)",
    };
    pub const DR_DIST_2ND_TRANSMISSION_BIT1: BitmapBit = BitmapBit {
        bit: 6,
        txt: "2nd tr: data rate distribution (Bit 1)",
    };
    pub const DR_DIST_2ND_TRANSMISSION_BIT2: BitmapBit = BitmapBit {
        bit: 7,
        txt: "2nd tr: data rate distribution (Bit 2)",
    };

    pub const FIRST_TRANSM_DR0_ENABLED: BitmapBit = BitmapBit {
        bit: 8,
        txt: "TX1 DR0/SF12-125kHz",
    };
    pub const FIRST_TRANSM_DR1_ENABLED: BitmapBit = BitmapBit {
        bit: 9,
        txt: "TX1 DR1/SF11-125kHz",
    };
    pub const FIRST_TRANSM_DR2_ENABLED: BitmapBit = BitmapBit {
        bit: 10,
        txt: "TX1 DR2/SF10-125kHz",
    };
    pub const FIRST_TRANSM_DR3_ENABLED: BitmapBit = BitmapBit {
        bit: 11,
        txt: "TX1 DR3/SF9-125kHz",
    };
    pub const FIRST_TRANSM_DR4_ENABLED: BitmapBit = BitmapBit {
        bit: 12,
        txt: "TX1 DR4/SF8-125kHz",
    };
    pub const FIRST_TRANSM_DR5_ENABLED: BitmapBit = BitmapBit {
        bit: 13,
        txt: "TX1 DR5/SF7-125kHz",
    };
    pub const FIRST_TRANSM_DR6_ENABLED: BitmapBit = BitmapBit {
        bit: 14,
        txt: "TX1 DR6/SF7-250kHz",
    };
    pub const FIRST_TRANSM_DR7_ENABLED: BitmapBit = BitmapBit {
        bit: 15,
        txt: "TX1 DR7/FSK-50kbps",
    };

    pub const SECOND_TRANSM_DR0_ENABLED: BitmapBit = BitmapBit {
        bit: 16,
        txt: "TX2 DR0/SF12-125kHz",
    };
    pub const SECOND_TRANSM_DR1_ENABLED: BitmapBit = BitmapBit {
        bit: 17,
        txt: "TX2 DR1/SF11-125kHz",
    };
    pub const SECOND_TRANSM_DR2_ENABLED: BitmapBit = BitmapBit {
        bit: 18,
        txt: "TX2 DR2/SF10-125kHz",
    };
    pub const SECOND_TRANSM_DR3_ENABLED: BitmapBit = BitmapBit {
        bit: 19,
        txt: "TX2 DR3/SF9-125kHz",
    };
    pub const SECOND_TRANSM_DR4_ENABLED: BitmapBit = BitmapBit {
        bit: 20,
        txt: "TX2 DR4/SF8-125kHz",
    };
    pub const SECOND_TRANSM_DR5_ENABLED: BitmapBit = BitmapBit {
        bit: 21,
        txt: "TX2 DR5/SF7-125kHz",
    };
    pub const SECOND_TRANSM_DR6_ENABLED: BitmapBit = BitmapBit {
        bit: 22,
        txt: "TX2 DR6/SF7-250kHz",
    };
    pub const SECOND_TRANSM_DR7_ENABLED: BitmapBit = BitmapBit {
        bit: 23,
        txt: "TX2 DR7/FSK-50kbps",
    };
}

// ***********************
// CONFIG_FLAGS
// ***********************

pub static CONFIG_FLAGS: PrmDatBitmap = PrmDatBitmap {
    id: 0x0d,
    name: "config_flags",
    label: "Configuration Flags",
    description: "Configuration Flags Help Text",
    default_val: 17252411,
    bits: &[
        ConfigFlagsBit::FRAME_PENDING_ENABLED,
        ConfigFlagsBit::LONG_BUTTON_PRESS_FOR_OFF,
        // ConfigFlagsBit::DEPRECATED_2,
        ConfigFlagsBit::CONF_DL_ACK,
        ConfigFlagsBit::WIFI_SCAN_ENC,
        ConfigFlagsBit::BLE_ACTIVE_AT_START,
        ConfigFlagsBit::WIFI_SCAN_ON_GEOLOC_START,
        ConfigFlagsBit::LED_BLINKS_ON_GPS_FIX,
        ConfigFlagsBit::MOTION_START_EVENT_ENABLED,
        ConfigFlagsBit::MOTION_END_EVENT_ENABLED,
        ConfigFlagsBit::JOIN_REQ_ON_LEAVING_OFF,
        ConfigFlagsBit::ASYMM_BLE_PAIRING_REJECTED,
        ConfigFlagsBit::LONG_WIFI_PAYLOAD,
        ConfigFlagsBit::COLLECTION_OF_LONG_REPORT,
        ConfigFlagsBit::AUTOSTART_ON_LEAVING_SHIPPING_STATE,
        ConfigFlagsBit::OFF_MODE_FORBIDDEN,
        ConfigFlagsBit::BEEPS_IN_SOS_MODE,
        ConfigFlagsBit::ADR_ON_TOO_LONG_PAYLOAD,
        ConfigFlagsBit::EXTENDED_POSITION_PAYLOAD,
        // ConfigFlagsBit::RESERVED_19,
        ConfigFlagsBit::CLI_OVER_BLE_ENABLED,
        ConfigFlagsBit::BLE_PASSKEY_ENABLED,
        ConfigFlagsBit::GNSS_FIX_AFTER_WIFI_SCAN_WHEN_STATIC,
        ConfigFlagsBit::WIFI_BEACONING_ENABLED,
        ConfigFlagsBit::CONTINUOUS_GNSS_OPERATION,
    ],
};
pub struct ConfigFlagsBit;
impl ConfigFlagsBit {
    pub const FRAME_PENDING_ENABLED: BitmapBit = BitmapBit {
        bit: 0,
        txt: "Frame pending mechanism is enabled",
    };
    pub const LONG_BUTTON_PRESS_FOR_OFF: BitmapBit = BitmapBit {
        bit: 1,
        txt: "Long button press activates OFF mode",
    };
    // pub const DEPRECATED_2: BitmapBit =
    // BitmapBit{bit: 2, txt: "Deprecated_2"};
    pub const CONF_DL_ACK: BitmapBit = BitmapBit {
        bit: 3,
        txt: "Config downlinks are acknowledged",
    };
    pub const WIFI_SCAN_ENC: BitmapBit = BitmapBit {
        bit: 4,
        txt: "WiFi scan payload is encrypted",
    };
    pub const BLE_ACTIVE_AT_START: BitmapBit = BitmapBit {
        bit: 5,
        txt: "BLE connectivity interface is active at start",
    };
    pub const WIFI_SCAN_ON_GEOLOC_START: BitmapBit = BitmapBit {
        bit: 6,
        txt: "Wi-Fi scan when the GNSS geoloc procdedure starts",
    };
    pub const LED_BLINKS_ON_GPS_FIX: BitmapBit = BitmapBit {
        bit: 7,
        txt: "LED blinks when a GPS fix is received",
    };
    pub const MOTION_START_EVENT_ENABLED: BitmapBit = BitmapBit {
        bit: 8,
        txt: "Motion Start event messages are enabled",
    };
    pub const MOTION_END_EVENT_ENABLED: BitmapBit = BitmapBit {
        bit: 9,
        txt: "Motion End event messages are enabled",
    };
    pub const JOIN_REQ_ON_LEAVING_OFF: BitmapBit = BitmapBit {
        bit: 10,
        txt: "A new Join Request is sent upon leaving OFF mode",
    };
    pub const ASYMM_BLE_PAIRING_REJECTED: BitmapBit = BitmapBit {
        bit: 11,
        txt: "Asymmetric BLE pairing is rejected",
    };
    pub const LONG_WIFI_PAYLOAD: BitmapBit = BitmapBit {
        bit: 12,
        txt: "Long Wi-Fi payload with up to 12 BSSID is enabled",
    };
    pub const COLLECTION_OF_LONG_REPORT: BitmapBit = BitmapBit {
        bit: 13,
        txt: "Collection of Long Report enabled",
    };
    pub const AUTOSTART_ON_LEAVING_SHIPPING_STATE: BitmapBit = BitmapBit {
        bit: 14,
        txt: "Autostart of the tracker when leaving shipping state",
    };
    pub const OFF_MODE_FORBIDDEN: BitmapBit = BitmapBit {
        bit: 15,
        txt: "OFF mode is forbidden",
    };
    pub const BEEPS_IN_SOS_MODE: BitmapBit = BitmapBit {
        bit: 16,
        txt: "Beeps are played during SOS mode",
    };
    pub const ADR_ON_TOO_LONG_PAYLOAD: BitmapBit = BitmapBit {
        bit: 17,
        txt: "ADR is enabled in case of too long payloads",
    };
    pub const EXTENDED_POSITION_PAYLOAD: BitmapBit = BitmapBit {
        bit: 18,
        txt: "Extended position payload enabled",
    };
    // pub const RESERVED_19: BitmapBit =
    // BitmapBit{bit: 19, txt: "Reserved_19"};
    pub const CLI_OVER_BLE_ENABLED: BitmapBit = BitmapBit {
        bit: 20,
        txt: "CLI over BLE is enabled",
    };
    pub const BLE_PASSKEY_ENABLED: BitmapBit = BitmapBit {
        bit: 21,
        txt: "BLE passkey authentication is enabled",
    };
    pub const GNSS_FIX_AFTER_WIFI_SCAN_WHEN_STATIC: BitmapBit = BitmapBit {
        bit: 22,
        txt: "GNSS fix after WiFi scan when static",
    };
    pub const WIFI_BEACONING_ENABLED: BitmapBit = BitmapBit {
        bit: 23,
        txt: "Wi-Fi beaconing enabled",
    };
    pub const CONTINUOUS_GNSS_OPERATION: BitmapBit = BitmapBit {
        bit: 24,
        txt: "Continuous GNSS operation enabled",
    };
}
