use super::param_comp_type_props::ParamOptionVariant;

pub struct ModeOption;
impl ModeOption {
    pub const STANDBY: ParamOptionVariant = ParamOptionVariant {
        value: 0,
        text: "Standby",
    };
    pub const MOTION_TRACKING: ParamOptionVariant = ParamOptionVariant {
        value: 1,
        text: "Motion tracking",
    };
    pub const PERMANENT_TRACKING: ParamOptionVariant = ParamOptionVariant {
        value: 2,
        text: "Permanent tracking",
    };
    pub const MOTION_START_END_TRACKING: ParamOptionVariant = ParamOptionVariant {
        value: 3,
        text: "Motion start/end tracking",
    };
    pub const ACTIVITY_TRACKING: ParamOptionVariant = ParamOptionVariant {
        value: 4,
        text: "Activity tracking",
    };
    pub const OFF: ParamOptionVariant = ParamOptionVariant {
        value: 5,
        text: "Off",
    };
    pub const VARIANTS: &'static [ParamOptionVariant] = &[
        Self::STANDBY,
        Self::MOTION_TRACKING,
        Self::PERMANENT_TRACKING,
        Self::MOTION_START_END_TRACKING,
        Self::ACTIVITY_TRACKING,
        Self::OFF,
    ];
}

pub struct GeolocSensorOption;
impl GeolocSensorOption {
    pub const WIFI: ParamOptionVariant = ParamOptionVariant {
        value: 0,
        text: "WiFi",
    };
    pub const GPS: ParamOptionVariant = ParamOptionVariant {
        value: 1,
        text: "GPS",
    };
    pub const LPGPS: ParamOptionVariant = ParamOptionVariant {
        value: 2,
        text: "LPGPS",
    };
    pub const RESERVED_3: ParamOptionVariant = ParamOptionVariant {
        value: 3,
        text: "Reserved_3",
    };
    pub const RESERVED_4: ParamOptionVariant = ParamOptionVariant {
        value: 4,
        text: "Reserved_4",
    };
    pub const RESERVED_5: ParamOptionVariant = ParamOptionVariant {
        value: 5,
        text: "Reserved_5",
    };
    pub const WIFI_GPS: ParamOptionVariant = ParamOptionVariant {
        value: 6,
        text: "WIFI-GPS",
    };
    pub const WIFI_LPGPS: ParamOptionVariant = ParamOptionVariant {
        value: 7,
        text: "WIFI-LPGPS",
    };
    pub const RESERVED_8: ParamOptionVariant = ParamOptionVariant {
        value: 8,
        text: "Reserved_8",
    };
    pub const WIFI_LPGPS_WIFI_GPS: ParamOptionVariant = ParamOptionVariant {
        value: 9,
        text: "WIFI-LPGPS/WIFI-GPS",
    };
    pub const BLE: ParamOptionVariant = ParamOptionVariant {
        value: 10,
        text: "BLE",
    };
    pub const BLE_GPS: ParamOptionVariant = ParamOptionVariant {
        value: 11,
        text: "BLE-GPS",
    };
    pub const BLE_LPGPS: ParamOptionVariant = ParamOptionVariant {
        value: 12,
        text: "BLE-LPGPS",
    };

    pub const VARIANTS: &'static [ParamOptionVariant] = &[
        Self::WIFI,
        Self::GPS,
        Self::LPGPS,
        // Self::RESERVED_3, Self::RESERVED_4, Self::RESERVED_5,
        Self::WIFI_GPS,
        Self::WIFI_LPGPS,
        // Self::RESERVED_8,
        Self::WIFI_LPGPS_WIFI_GPS,
        Self::BLE,
        Self::BLE_GPS,
        Self::BLE_LPGPS,
    ];
}

pub struct GeolocMethodOption;
impl GeolocMethodOption {
    pub const WIFI: ParamOptionVariant = ParamOptionVariant {
        value: 0,
        text: "WiFi",
    };
    pub const GPS: ParamOptionVariant = ParamOptionVariant {
        value: 1,
        text: "GPS",
    };
    pub const LPGPS: ParamOptionVariant = ParamOptionVariant {
        value: 2,
        text: "LPGPS",
    };
    pub const WIFI_GPS: ParamOptionVariant = ParamOptionVariant {
        value: 3,
        text: "WIFI-GPS",
    };
    pub const WIFI_LPGPS: ParamOptionVariant = ParamOptionVariant {
        value: 4,
        text: "WIFI-LPGPS",
    };
    pub const BLE: ParamOptionVariant = ParamOptionVariant {
        value: 5,
        text: "BLE",
    };
    pub const BLE_GPS: ParamOptionVariant = ParamOptionVariant {
        value: 6,
        text: "BLE-GPS",
    };
    pub const BLE_LPGPS: ParamOptionVariant = ParamOptionVariant {
        value: 7,
        text: "BLE-LPGPS",
    };

    pub const VARIANTS: &'static [ParamOptionVariant] = &[
        Self::WIFI,
        Self::GPS,
        Self::LPGPS,
        Self::WIFI_GPS,
        Self::WIFI_LPGPS,
        Self::BLE,
        Self::BLE_GPS,
        Self::BLE_LPGPS,
    ];
}

pub struct TransmitStratOption;
impl TransmitStratOption {
    pub const SINGLE_FIXED: ParamOptionVariant = ParamOptionVariant {
        value: 0,
        text: "Single fixed",
    };
    pub const SINGLE_RANDOM: ParamOptionVariant = ParamOptionVariant {
        value: 1,
        text: "Single random",
    };
    pub const DOUBLE_RANDOM: ParamOptionVariant = ParamOptionVariant {
        value: 2,
        text: "Double random",
    };
    pub const DOUBLE_FIXED: ParamOptionVariant = ParamOptionVariant {
        value: 3,
        text: "Doubl fixed",
    };
    // pub const RESERVED: ConfParamOptionVariant = ConfParamOptionVariant{
    //     value: 4,
    //     text: "Reserved"
    // };
    pub const CUSTOM: ParamOptionVariant = ParamOptionVariant {
        value: 5,
        text: "Custom",
    };
    pub const VARIANTS: &'static [ParamOptionVariant] = &[
        Self::SINGLE_FIXED,
        Self::SINGLE_RANDOM,
        Self::DOUBLE_RANDOM,
        Self::DOUBLE_FIXED,
        // Self::RESERVED,
        Self::CUSTOM,
    ];
}
