use crate::prm::typ::{
    BitmapBit, DistinctVal, PrmDatBitmap, PrmDatDec, PrmDatDistinct, PrmDatOptional,
};

use super::typ::{PrmDatBatteryCapacity, PrmDatButtonMapping, PrmDatMotionSensitivity};


// *************************************************
// ***                                           ***
// ***   Parameters for main operational modes   ***
// ***                                           ***
// *************************************************

// ***********************
// *** 249 - MODE
// ***********************

pub static MODE: PrmDatDistinct = PrmDatDistinct {
    id: 0xf9,
    name: "mode",
    label: "Operating Mode",
    description: "The operation mode of the tracker.",
    default_val: ModeOption::MOTION_TRACKING.val,
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
// *** 101 - DEFAULT_PROFILE
// ***********************

pub static DEFAULT_PROFILE: PrmDatDistinct = PrmDatDistinct {
    id: 0x65,
    name: "default_profile",
    label: "Default profile",
    description: "Profile applicable to configure several parameters at once, used when application starts, after a reset or when the tracker is turned on",
    default_val: DefaultProfileOption::NONE.val,
    distinct_vals: &[
        DefaultProfileOption::NONE,
        DefaultProfileOption::SLEEP,
        DefaultProfileOption::ECONOMIC,
        DefaultProfileOption::INTENSIVE,
    ],
};
pub struct DefaultProfileOption;
impl DefaultProfileOption {
    pub const NONE: DistinctVal = DistinctVal {
        val: 0,
        txt: "None",
    };
    pub const SLEEP: DistinctVal = DistinctVal {
        val: 1,
        txt: "Sleep",
    };
    pub const ECONOMIC: DistinctVal = DistinctVal {
        val: 2,
        txt: "Economic",
    };
    pub const INTENSIVE: DistinctVal = DistinctVal {
        val: 3,
        txt: "Intensive",
    };
}


// ****************************************
// *** 246 - PROFILE - SPECIAL
// ****************************************

pub static PROFILE: PrmDatDistinct = PrmDatDistinct {
    id: 0xF6,
    name: "profile",
    label: "Dynamic profile",
    description: "Sets the dynamic profile of the tracker.",
    default_val: ProfileOption::NONE.val,
    distinct_vals: &[
        ProfileOption::NONE,
        ProfileOption::SLEEP,
        ProfileOption::ECONOMIC,
        ProfileOption::INTENSIVE,
    ],
};
pub struct ProfileOption;
impl ProfileOption {
    pub const NONE: DistinctVal = DistinctVal {
        val: 0,
        txt: "None",
    };
    pub const SLEEP: DistinctVal = DistinctVal {
        val: 1,
        txt: "Sleep",
    };
    pub const ECONOMIC: DistinctVal = DistinctVal {
        val: 2,
        txt: "Economic",
    };
    pub const INTENSIVE: DistinctVal = DistinctVal {
        val: 3,
        txt: "Intensive",
    };
}


// ***********************
// 0 - UL_PERIOD
// ***********************

pub static UL_PERIOD: PrmDatDec = PrmDatDec {
    id: 0x00,
    name: "ul_period",
    label: "Report Period",
    description: "Periodicity of position or activity messages in motion, start/end, activity or permanent tracking operating mode. [15..86400][s]",
    default_val: 300,
    range: (15, 86400),
};


// ***********************
// 1 - LORA_PERIOD
// ***********************

pub static LORA_PERIOD: PrmDatDec = PrmDatDec {
    id: 0x01,
    name: "lora_period",
    label: "Heartbeat period",
    description: "Periodicity of LoRa heartbeat messages. [300..86400][s]",
    default_val: 600,
    range: (300, 86400),
};


// ***********************
// 5 - GEOLOC_SENSOR
// ***********************

pub static GEOLOC_SENSOR: PrmDatDistinct = PrmDatDistinct {
    id: 0x05,
    name: "geoloc_sensor",
    label: "Main Geoloc Techn",
    description: "Geolocation technology used in main operating mode and SOS.",
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
// 117 - SOS_PERIOD
// ***********************

pub static SOS_PERIOD: PrmDatDec = PrmDatDec {
    id: 0x75,
    name: "sos_period",
    label: "SOS Period",
    description: "Period of SOS messages.  [15..300][s]",
    default_val: 120,
    range: (15, 300),
};



// *************************************************
// ***                                           ***
// ***   Parameters for side operational modes   ***
// ***                                           ***
// *************************************************

// ***********************
// 3 - PERIODIC_POS_PERIOD
// ***********************

pub static PERIODIC_POS_PERIOD: PrmDatOptional = PrmDatOptional {
    id: 0x03,
    name: "periodic_pos_period",
    label: "Periodic Position Period",
    description: "Periodic Position Report Side Operation Period. [900..604800][s]",
    default_val: 14400,
    disabled_val: 0,
    range: (900 , 604800),
};


// ***********************
// 6 - GEOLOC_METHOD
// ***********************

pub static GEOLOC_METHOD: PrmDatDistinct = PrmDatDistinct {
    id: 0x06,
    name: "geoloc_method",
    label: "Side Geoloc Techn",
    description: "Geolocation technology used in side operating modes.",
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
    pub const GPS: DistinctVal = DistinctVal { 
        val: 1, 
        txt: "GPS" };
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
// 22 - PERIODIC_ACTIVITY_PERIOD
// ***********************

pub static PERIODIC_ACTIVITY_PERIOD: PrmDatOptional = PrmDatOptional {
    id: 0x16,
    name: "periodic_activity_period",
    label: "Periodic Activity Period",
    description: "Periodic activity report period. The value is rounded up to the closest multiple of 6. [1800 , 86400][s]",
    default_val: 0,
    disabled_val: 0,
    range: (1800 , 86400),
};


// *************************************************
// ***                                           ***
// ***   Parameters for collections              ***
// ***                                           ***
// *************************************************



// ***********************
// 33 - COLLECTION_SCAN_TYPE
// ***********************

pub static COLLECTION_SCAN_TYPE: PrmDatDistinct = PrmDatDistinct {
    id: 0x21,
    name: "collection_scan_type",
    label: "BLE Scan Collection Type",
    description: "Collection scan type used",
    default_val: CollScanTypeOption::NONE.val,
    distinct_vals: &[
        CollScanTypeOption::NONE,
        CollScanTypeOption::BLE,
        CollScanTypeOption::WIFI,
    ],
};
pub struct CollScanTypeOption;
impl CollScanTypeOption {
    pub const NONE: DistinctVal = DistinctVal {
        val: 0,
        txt: "Disabled",
    };
    pub const BLE: DistinctVal = DistinctVal {
        val: 1,
        txt: "BLE collection scan",
    };
    pub const WIFI: DistinctVal = DistinctVal {
        val: 2,
        txt: "Wi-Fi collection scan",
    };

}


// ***********************
// 34 - COLLECTION_NB_ENTRY
// ***********************

pub static COLLECTION_NB_ENTRY: PrmDatDec = PrmDatDec {
    id: 0x22,
    name: "collection_nb_entry",
    label: "Max num of collected elements",
    description: "Maximum number of elements to report in collection payloads after a scan. [1..20][]",
    default_val: 20,
    range: (1, 20),
};


// ***********************
// 35 - COLLECTION_BLE_FILTER_TYPE
// ***********************

pub static COLLECTION_BLE_FILTER_TYPE: PrmDatDistinct = PrmDatDistinct {
    id: 0x23,
    name: "collection_ble_filter_type",
    label: "BLE Filter Type - Collection",
    description: "Beacon type to scan and report when Collection Scan Type is BLE.",
    default_val: BleFilterTypeOption::NONE.val,
    distinct_vals: &[
        BleFilterTypeOption::NONE,
        BleFilterTypeOption::EDYSTONE_UID,
        BleFilterTypeOption::EDYSTONE_URL,
        BleFilterTypeOption::ALL_EDYSTONE,
        BleFilterTypeOption::I_BEACON_UID,
        BleFilterTypeOption::ALT_BEACON,
        // CollBleFilterTypeOption::RESERVED,
    ],
};
pub struct BleFilterTypeOption;
impl BleFilterTypeOption {
    pub const NONE: DistinctVal = DistinctVal {
        val: 0,
        txt: "No filter",
    };
    pub const EDYSTONE_UID: DistinctVal = DistinctVal {
        val: 1,
        txt: "Eddystone UID only",
    };
    pub const EDYSTONE_URL: DistinctVal = DistinctVal {
        val: 2,
        txt: "Eddystone URL only",
    };
    pub const ALL_EDYSTONE: DistinctVal = DistinctVal {
        val: 3,
        txt: "All Eddystone",
    };
    pub const I_BEACON_UID: DistinctVal = DistinctVal {
        val: 4,
        txt: "iBeacon UID only",
    };
    pub const ALT_BEACON: DistinctVal = DistinctVal {
        val: 5,
        txt: "altBeacon only",
    };
    // pub const RESERVED: DistinctVal = DistinctVal {
    //     val: 6,
    //     txt: "Reserved, internal use only",
    // };

}


// ***********************
// 36 - COLLECTION_BLE_FILTER_MAIN_1
// ***********************

pub static COLLECTION_BLE_FILTER_MAIN_1: PrmDatOptional = PrmDatOptional {
    id: 0x24,
    name: "collection_ble_filter_main_1",
    label: "Main BLE Filter 1 - Collection",
    description: "Main BLE filter for Collection messages - 1st byte.",
    default_val: 0,
    disabled_val: 0,
    range: (i32::MIN, i32::MAX),
};


// ***********************
// 37 - COLLECTION_BLE_FILTER_MAIN_2
// ***********************

pub static COLLECTION_BLE_FILTER_MAIN_2: PrmDatOptional = PrmDatOptional {
    id: 0x25,
    name: "collection_ble_filter_main_2",
    label: "Main BLE Filter 1 - Collection",
    description: "Main BLE filter for Collection messages - 2nd byte.",
    default_val: 0,
    disabled_val: 0,
    range: (i32::MIN, i32::MAX),
};


// ***********************
// 38 - COLLECTION_BLE_FILTER_SEC_VALUE
// ***********************

pub static COLLECTION_BLE_FILTER_SEC_VALUE: PrmDatOptional = PrmDatOptional {
    id: 0x26,
    name: "collection_ble_filter_sec_value",
    label: "Sec. BLE Filter Value - Collection",
    description: "Secondary BLE filter VALUE for Collection messages.",
    default_val: 0,
    disabled_val: 0,
    range: (i32::MIN, i32::MAX),
};


// ***********************
// 39 - COLLECTION_BLE_FILTER_SEC_MASK
// ***********************

pub static COLLECTION_BLE_FILTER_SEC_MASK: PrmDatOptional = PrmDatOptional {
    id: 0x27,
    name: "collection_ble_filter_sec_mask",
    label: "Sec. BLE Filter Mask - Collection",
    description: "Secondary BLE filter MASK for Collection messages.",
    default_val: 0,
    disabled_val: 0,
    range: (i32::MIN, i32::MAX),
};




// ******************************************************************
// ***                                                            ***
// ***   Parameters for GPS and low power GPS geolocation modes   ***
// ***                                                            ***
// ******************************************************************


// ***********************
// 42 - GNSS_CONSTELLATION
// ***********************

pub static GNSS_CONSTELLATION: PrmDatDistinct = PrmDatDistinct {
    id: 0x2A,
    name: "gnss_constellation",
    label: "GNSS Constellation",
    description: "The GNSS constellations used to compute a position",
    default_val: GeolocSensorOption::GPS.val,
    distinct_vals: &[
        GnssConstellation::GPS,
        GnssConstellation::GLONASS,
        GnssConstellation::GPS_GLONASS,
        GnssConstellation::GPS_GALILEO,
        GnssConstellation::GPS_GLONASS_GALILEO,
        GnssConstellation::BEIDOU,
        GnssConstellation::GPS_BEIDOU,
    ],
};
pub struct GnssConstellation;
impl GnssConstellation {
    pub const GPS: DistinctVal = DistinctVal {
        val: 0,
        txt: "GPS",
    };
    pub const GLONASS: DistinctVal = DistinctVal { 
        val: 1, 
        txt: "GLONASS" 
    };
    pub const GPS_GLONASS: DistinctVal = DistinctVal {
        val: 2,
        txt: "GPS and GLONASS",
    };
    pub const GPS_GALILEO: DistinctVal = DistinctVal {
        val: 3,
        txt: "GPS and Galileo",
    };
    pub const GPS_GLONASS_GALILEO: DistinctVal = DistinctVal { 
        val: 4, 
        txt: "GPS, GLONASS and Galileo" 
    };
    pub const BEIDOU: DistinctVal = DistinctVal {
        val: 5,
        txt: "Beidou",
    };
    pub const GPS_BEIDOU: DistinctVal = DistinctVal {
        val: 6,
        txt: "GPS and BEIDOU",
    };
}


// ***********************
// 9 - GPS_TIMEOUT
// ***********************

pub static GPS_TIMEOUT: PrmDatDec = PrmDatDec {
    id: 0x09,
    name: "gps_timeout",
    label: "GPS Timeout",
    description: "The GPS geolocalization procedure will be cancelled after tis timeout and either a Position Message or a Timeout Message will be sent. [30..300][s]",
    default_val: 240,
    range: (30, 300),
};


// ***********************
// 11 - GPS_EHPE
// ***********************

pub static GPS_EHPE: PrmDatDec = PrmDatDec {
    id: 0x0B,
    name: "gps_ehpe",
    label: "GPS Postion Error - Static",
    description: "Acceptable GPS horizontal postion error in static mode. [0..100][m]",
    default_val: 20,
    range: (0, 100),
};


// ***********************
// 121 - GPS_EHPE_MOTION
// ***********************

pub static GPS_EHPE_MOTION: PrmDatDec = PrmDatDec {
    id: 0x79,
    name: "gps_ehpe_motion",
    label: "GPS Postion Error - Motion",
    description: "Acceptable GPS horizontal postion error in motion. [0..100][m]",
    default_val: 40,
    range: (0, 100),
};


// ***********************
// 12 - GPS_CONVERGENCE
// ***********************

pub static GPS_CONVERGENCE: PrmDatDec = PrmDatDec {
    id: 0x0C,
    name: "gps_convergence",
    label: "GPS Convergence Time - Static",
    description: "Time to let to the GPS module to refine the calculated GPS position in static mode. [0..300][s]",
    default_val: 30,
    range: (0, 300),
};


// ***********************
// 0x7A - GPS_CONVERGENCE_MOTION
// ***********************

pub static GPS_CONVERGENCE_MOTION: PrmDatDec = PrmDatDec {
    id: 0x7A,
    name: "gps_convergence_motion",
    label: "GPS Convergence Time - Motion",
    description: "Time to let to the GPS module to refine the calculated GPS position in motion. This parameter will overwrite ul_period. Set it to the same or smaller than ul_period. [0..300][s]",
    default_val: 20,
    range: (0, 300),
};


// ***********************
// 17 - GPS_STANDBY_TIMEOUT
// ***********************

pub static GPS_STANDBY_TIMEOUT: PrmDatOptional = PrmDatOptional {
    id: 0x11,
    name: "gps_standby_timeout",
    label: "GPS Standby Timeout",
    description: "Duration of the GPS standby mode before going OFF. [1..2147483647][s]",
    default_val: 0,
    disabled_val: 0,
    range: (1, 2147483647),
};


// ***********************
// 103 - GPS_T0_TIMEOUT
// ***********************

pub static GPS_T0_TIMEOUT: PrmDatOptional = PrmDatOptional {
    id: 0x67,
    name: "gps_t0_timeout",
    label: "GPS T0 Timeout - Static",
    description: "Timeout to abort the GPS or LPGPS geolocation when not enough satellites with C/N > 15 are in view. Applicable if the tracker is static. [1..300][s]",
    default_val: 15,
    disabled_val: 0,
    range: (1, 300),
};


// ***********************
// 123 - GPS_T0_TIMEOUT_MOTION
// ***********************

pub static GPS_T0_TIMEOUT_MOTION: PrmDatOptional = PrmDatOptional {
    id: 0x7B,
    name: "gps_t0_timeout_motion",
    label: "GPS T0 Timeout - Motion",
    description: "Timeout to abort the GPS or LPGPS geolocation when not enough satellites with C/N > 15 are in view. Applicable if the tracker is in motion. [1..300][s]",
    default_val: 30,
    disabled_val: 0,
    range: (1, 300),
};


// ***********************
// 104 - GPS_FIX_TIMEOUT
// ***********************

pub static GPS_FIX_TIMEOUT: PrmDatOptional = PrmDatOptional {
    id: 0x68,
    name: "gps_fix_timeout",
    label: "GPS Fix Timeout",
    description: "Timeout to abort the GPS geolocation (or to switch to LPGPS) if there is no GPS fix. [1..300][s]",
    default_val: 0,
    disabled_val: 0,
    range: (1, 300),
};


// ***********************
// 10 - AGPS_TIMEOUT
// ***********************

// Timeout used for LPGPS geolocation mode before sending the geolocation message (GPS/LP-GPS position or timeout)

pub static AGPS_TIMEOUT: PrmDatDec = PrmDatDec {
    id: 0x0A,
    name: "agps_timeout",
    label: "LPGPS Timeout",
    description: "Timeout of LPGPS geolocation before sending a position update message (GPS position / LP-GPS position / Timeout) [15..250][s]",
    default_val: 45,
    range: (15, 250),
};





// *************************************************
// ***                                           ***
// ***   LoRa Parameters                         ***
// ***                                           ***
// *************************************************


// ***********************
// 14 - TRANSMIT_STRAT
// ***********************

pub static TRANSMIT_STRAT: PrmDatDistinct = PrmDatDistinct {
    id: 0x0e,
    name: "transmit_strat",
    label: "Transmit Strategy",
    description: "LoRaWAN Transmit Strategy.",
    default_val: TransmitStratOption::DOUBLE_RANDOM.val,
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
    default_val: 0,
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
        ena: true,
    };
    pub const DOUBLE_TRANSMISSION: BitmapBit = BitmapBit {
        bit: 1,
        txt: "Double transmission enabled",
        ena: true,
    };

    pub const DR_DIST_1ST_TRANSMISSION_BIT0: BitmapBit = BitmapBit {
        bit: 2,
        txt: "1st tr: data rate distribution (Bit 0)",
        ena: true,
    };
    pub const DR_DIST_1ST_TRANSMISSION_BIT1: BitmapBit = BitmapBit {
        bit: 3,
        txt: "1st tr: data rate distribution (Bit 1)",
        ena: true,
    };
    pub const DR_DIST_1ST_TRANSMISSION_BIT2: BitmapBit = BitmapBit {
        bit: 4,
        txt: "1st tr: data rate distribution (Bit 2)",
        ena: true,
    };

    pub const DR_DIST_2ND_TRANSMISSION_BIT0: BitmapBit = BitmapBit {
        bit: 5,
        txt: "2nd tr: data rate distribution (Bit 0)",
        ena: true,
    };
    pub const DR_DIST_2ND_TRANSMISSION_BIT1: BitmapBit = BitmapBit {
        bit: 6,
        txt: "2nd tr: data rate distribution (Bit 1)",
        ena: true,
    };
    pub const DR_DIST_2ND_TRANSMISSION_BIT2: BitmapBit = BitmapBit {
        bit: 7,
        txt: "2nd tr: data rate distribution (Bit 2)",
        ena: true,
    };

    pub const FIRST_TRANSM_DR0_ENABLED: BitmapBit = BitmapBit {
        bit: 8,
        txt: "TX1 DR0/SF12-125kHz",
        ena: true,
    };
    pub const FIRST_TRANSM_DR1_ENABLED: BitmapBit = BitmapBit {
        bit: 9,
        txt: "TX1 DR1/SF11-125kHz",
        ena: true,
    };
    pub const FIRST_TRANSM_DR2_ENABLED: BitmapBit = BitmapBit {
        bit: 10,
        txt: "TX1 DR2/SF10-125kHz",
        ena: true,
    };
    pub const FIRST_TRANSM_DR3_ENABLED: BitmapBit = BitmapBit {
        bit: 11,
        txt: "TX1 DR3/SF9-125kHz",
        ena: true,
    };
    pub const FIRST_TRANSM_DR4_ENABLED: BitmapBit = BitmapBit {
        bit: 12,
        txt: "TX1 DR4/SF8-125kHz",
        ena: true,
    };
    pub const FIRST_TRANSM_DR5_ENABLED: BitmapBit = BitmapBit {
        bit: 13,
        txt: "TX1 DR5/SF7-125kHz",
        ena: true,
    };
    pub const FIRST_TRANSM_DR6_ENABLED: BitmapBit = BitmapBit {
        bit: 14,
        txt: "TX1 DR6/SF7-250kHz",
        ena: true,
    };
    pub const FIRST_TRANSM_DR7_ENABLED: BitmapBit = BitmapBit {
        bit: 15,
        txt: "TX1 DR7/FSK-50kbps",
        ena: true,
    };

    pub const SECOND_TRANSM_DR0_ENABLED: BitmapBit = BitmapBit {
        bit: 16,
        txt: "TX2 DR0/SF12-125kHz",
        ena: true,
    };
    pub const SECOND_TRANSM_DR1_ENABLED: BitmapBit = BitmapBit {
        bit: 17,
        txt: "TX2 DR1/SF11-125kHz",
        ena: true,
    };
    pub const SECOND_TRANSM_DR2_ENABLED: BitmapBit = BitmapBit {
        bit: 18,
        txt: "TX2 DR2/SF10-125kHz",
        ena: true,
    };
    pub const SECOND_TRANSM_DR3_ENABLED: BitmapBit = BitmapBit {
        bit: 19,
        txt: "TX2 DR3/SF9-125kHz",
        ena: true,
    };
    pub const SECOND_TRANSM_DR4_ENABLED: BitmapBit = BitmapBit {
        bit: 20,
        txt: "TX2 DR4/SF8-125kHz",
        ena: true,
    };
    pub const SECOND_TRANSM_DR5_ENABLED: BitmapBit = BitmapBit {
        bit: 21,
        txt: "TX2 DR5/SF7-125kHz",
        ena: true,
    };
    pub const SECOND_TRANSM_DR6_ENABLED: BitmapBit = BitmapBit {
        bit: 22,
        txt: "TX2 DR6/SF7-250kHz",
        ena: true,
    };
    pub const SECOND_TRANSM_DR7_ENABLED: BitmapBit = BitmapBit {
        bit: 23,
        txt: "TX2 DR7/FSK-50kbps",
        ena: true,
    };
    // TODO: complete until bit 31
}


// ***********************
// 120 - DEFAULT_DATARATE
// ***********************

pub static DEFAULT_DATARATE: PrmDatDistinct = PrmDatDistinct {
    id: 0x78,
    name: "default_datarate",
    label: "Default Data Rate",
    description: "Default Data Rate. If the selected DR is not supported by the Lora MAC the lowest supported one is used instead.",
    default_val: DefaultDROption::PROVISIONED.val,
    distinct_vals: &[
        DefaultDROption::PROVISIONED,
        DefaultDROption::DR0,
        DefaultDROption::DR1,
        DefaultDROption::DR2,
        DefaultDROption::DR3,
        DefaultDROption::DR4,
        DefaultDROption::DR5,
        DefaultDROption::DR6,
        DefaultDROption::DR7,
    ],
};
pub struct DefaultDROption;
impl DefaultDROption {
    pub const PROVISIONED: DistinctVal = DistinctVal {
        val: -1,
        txt: "Provisioned",
    };
    pub const DR0: DistinctVal = DistinctVal {
        val: 0,
        txt: "DR0/SF12-125kHz",
    };
    pub const DR1: DistinctVal = DistinctVal {
        val: 1,
        txt: "DR1/SF11-125kHz",
    };
    pub const DR2: DistinctVal = DistinctVal {
        val: 2,
        txt: "DR2/SF10-125kHz",
    };
    pub const DR3: DistinctVal = DistinctVal {
        val: 3,
        txt: "DR3/SF9-125kHz",
    };
    pub const DR4: DistinctVal = DistinctVal {
        val: 4,
        txt: "DR4/SF8-125kHz",
    };
    pub const DR5: DistinctVal = DistinctVal {
        val: 5,
        txt: "DR5/SF7-125kHz",
    };
    pub const DR6: DistinctVal = DistinctVal {
        val: 6,
        txt: "DR6/SF7-250kHz",
    };
    pub const DR7: DistinctVal = DistinctVal {
        val: 7,
        txt: "DR7/FSK-50kbps",
    };
}


// ***********************
// 18 CONFIRMED_UL_BITMAP
// ***********************

pub static CONFIRMED_UL_BITMAP: PrmDatBitmap = PrmDatBitmap {
    id: 0x12,
    name: "confirmed_ul_bitmap",
    label: "Confirmed Uplink Bitmap",
    description: "Bitmap enabling the LoRaWAN confirmation of specific types of uplink messages.",
    default_val: 0,
    bits: &[
        ConfirmedUplinkBit::FRAME_PENDING,

        ConfirmedUplinkBit::INVALID_1,
        ConfirmedUplinkBit::INVALID_2,

        ConfirmedUplinkBit::POSITION,
        ConfirmedUplinkBit::STATUS,
        ConfirmedUplinkBit::HEARTBEAT,

        ConfirmedUplinkBit::INVALID_6,

        ConfirmedUplinkBit::ACT_CONF_SHOCK_BLEMAC,

        ConfirmedUplinkBit::INVALID_8,

        ConfirmedUplinkBit::SHUTDOWN,
        ConfirmedUplinkBit::EVENT,
        ConfirmedUplinkBit::COLLECTION_SCAN,

        ConfirmedUplinkBit::INVALID_12,
        ConfirmedUplinkBit::INVALID_13,

        ConfirmedUplinkBit::EXTENDED_POSITION,
        ConfirmedUplinkBit::DEBUG,
    ],
};
pub struct ConfirmedUplinkBit;
impl ConfirmedUplinkBit {
    pub const FRAME_PENDING: BitmapBit = BitmapBit {
        bit: 0,
        txt: "Frame Pending Message",
        ena: true,
    };

    // 1, 2, 
    pub const INVALID_1: BitmapBit = BitmapBit {
        bit: 1,
        txt: "Invalid 1",
        ena: false,
    };
    pub const INVALID_2: BitmapBit = BitmapBit {
        bit: 2,
        txt: "Invalid 2",
        ena: false,
    };

    pub const POSITION: BitmapBit = BitmapBit {
        bit: 3,
        txt: "Position Message",
        ena: true,
    };
    pub const STATUS: BitmapBit = BitmapBit {
        bit: 4,
        txt: "Status Message",
        ena: true,
    };
    pub const HEARTBEAT: BitmapBit = BitmapBit {
        bit: 5,
        txt: "Heartbeat Message",
        ena: true,
    };

    // 6,
    pub const INVALID_6: BitmapBit = BitmapBit {
        bit: 6,
        txt: "Invalid 6",
        ena: false,
    };

    pub const ACT_CONF_SHOCK_BLEMAC: BitmapBit = BitmapBit {
        bit: 7,
        txt: "Activity, Config, Shock detec or BLE MAC Addr",
        ena: true,
    };

    // 8,
    pub const INVALID_8: BitmapBit = BitmapBit {
        bit: 8,
        txt: "Invalid 8",
        ena: false,
    };

    pub const SHUTDOWN: BitmapBit = BitmapBit {
        bit: 9,
        txt: "Shutdown message",
        ena: true,
    };
    pub const EVENT: BitmapBit = BitmapBit {
        bit: 10,
        txt: "Event message",
        ena: true,
    };
    pub const COLLECTION_SCAN: BitmapBit = BitmapBit {
        bit: 11,
        txt: "Collection Scan message",
        ena: true,
    };

    // 12, 13,
    pub const INVALID_12: BitmapBit = BitmapBit {
        bit: 12,
        txt: "Invalid 12",
        ena: false,
    };
    pub const INVALID_13: BitmapBit = BitmapBit {
        bit: 13,
        txt: "Invalid 12",
        ena: false,
    };

    pub const EXTENDED_POSITION: BitmapBit = BitmapBit {
        bit: 14,
        txt: "Extended Position message",
        ena: true,
    };
    pub const DEBUG: BitmapBit = BitmapBit {
        bit: 15,
        txt: "Debug for internal use",
        ena: false,
    };

}


// ***********************
// 19 - CONFIRMED_UL_RETRY
// ***********************

pub static CONFIRMED_UL_RETRY: PrmDatDec = PrmDatDec {
    id: 0x13,
    name: "confirmed_ul_retry",
    label: "Confirmed Uplink Retry",
    description: "The number of retries for each confirmed uplink when the confirmation is not received. [0..8][]",
    default_val: 3,
    range: (0, 8),
};


// ***********************
// 31 - NETWORK_TIMEOUT_CHECK
// ***********************

pub static NETWORK_TIMEOUT_CHECK: PrmDatOptional = PrmDatOptional {
    id: 0x1F,
    name: "network_timeout_check",
    label: "Network Timeout Check",
    description: "Time without received downlink before asking a link check request. [86400..5184000][s]",
    default_val: 432000,
    disabled_val: 0,
    range: (86400, 5184000),
};


// ***********************
// 32 - NETWORK_TIMEOUT_CHECK
// ***********************

pub static NETWORK_TIMEOUT_RESET: PrmDatOptional = PrmDatOptional {
    id: 0x20,
    name: "network_timeout_reset",
    label: "Network Timeout Reset",
    description: "Time after network_timeout_check without received downlink before the tracker resets. [21600..2592000][s]",
    default_val: 172800,
    disabled_val: 0,
    range: (21600, 2592000),
};


// *************************************************
// ***                                           ***
// ***   BLE scan and communication parameters   ***
// ***                                           ***
// *************************************************


// ***
// *** BLE position scan parameters ***
// ***


// ***********************
// 15 - BLE_BEACON_CNT
// ***********************

pub static BLE_BEACON_CNT: PrmDatDec = PrmDatDec {
    id: 0x0F,
    name: "ble_beacon_cnt",
    label: "BLE Beacon Count",
    description: "The maximum number of BLE beacons to provide in payload. [1..4][]",
    default_val: 4,
    range: (1, 4),
};


// ***********************
// 16 - BLE_BEACON_TIMEOUT
// ***********************

pub static BLE_BEACON_TIMEOUT: PrmDatDec = PrmDatDec {
    id: 0x10,
    name: "ble_beacon_timeout",
    label: "BLE Scan Duration",
    description: "BLE Scan Duration. [1..21][s]",
    default_val: 4,
    range: (1, 21),
};


// ***********************
// 26 - BLE_RSSI_FILTER
// ***********************

pub static BLE_RSSI_FILTER: PrmDatDec = PrmDatDec {
    id: 0x1A,
    name: "ble_rssi_filter",
    label: "BLE RSSI Filter",
    description: "RSSI value to filter BLE beacons with BLE geolocation modes (only applicable for BLE-GPS & BLE-LPGPS). [-100..-40][dBm]",
    default_val: -85,
    range: (-100, -40),
};


// ***********************
// 77 POSITION_BLE_FILTER_TYPE
// ***********************

pub static POSITION_BLE_FILTER_TYPE: PrmDatDistinct = PrmDatDistinct {
    id: 0x4D,
    name: "position_ble_filter_type",
    label: "BLE Filter Type - Tracking",
    description: "Beacon type to scan and report when position Scan Type is BLE.",
    default_val: 0,
    distinct_vals: &[
        BleFilterTypeOption::NONE,
        BleFilterTypeOption::EDYSTONE_UID,
        BleFilterTypeOption::EDYSTONE_URL,
        BleFilterTypeOption::ALL_EDYSTONE,
        BleFilterTypeOption::I_BEACON_UID,
        BleFilterTypeOption::ALT_BEACON,
        // CollBleFilterTypeOption::RESERVED,
    ],
};


// ***********************
// 78 - POSITION_BLE_FILTER_MAIN_1
// ***********************

pub static POSITION_BLE_FILTER_MAIN_1: PrmDatOptional = PrmDatOptional {
    id: 0x4E,
    name: "position_ble_filter_main_1",
    label: "Main BLE Filter 1 - Tracking",
    description: "Main BLE filter for Position messages - 1st byte.",
    default_val: 0,
    disabled_val: 0,
    range: (i32::MIN, i32::MAX),
};


// ***********************
// 79 - POSITION_BLE_FILTER_MAIN_2
// ***********************

pub static POSITION_BLE_FILTER_MAIN_2: PrmDatOptional = PrmDatOptional {
    id: 0x4F,
    name: "position_ble_filter_main_2",
    label: "Main BLE Filter 2 - Tracking",
    description: "Main BLE filter for Position messages - 2nd byte.",
    default_val: 0,
    disabled_val: 0,
    range: (i32::MIN, i32::MAX),
};


// ***********************
// 80 - POSITION_BLE_FILTER_SEC_VALUE
// ***********************

pub static POSITION_BLE_FILTER_SEC_VALUE: PrmDatOptional = PrmDatOptional {
    id: 0x50,
    name: "position_ble_filter_sec_value",
    label: "Sec. BLE Filter Value - Tracking",
    description: "Secondary BLE filter VALUE for Position messages.",
    default_val: 0,
    disabled_val: 0,
    range: (i32::MIN, i32::MAX),
};


// ***********************
// 81 - POSITION_BLE_FILTER_SEC_MASK
// ***********************

pub static POSITION_BLE_FILTER_SEC_MASK: PrmDatOptional = PrmDatOptional {
    id: 0x51,
    name: "position_ble_filter_sec_mask",
    label: "Sec. BLE Filter Mask - Tracking",
    description: "Secondary BLE filter MASK for Position messages.",
    default_val: 0,
    disabled_val: 0,
    range: (i32::MIN, i32::MAX),
};


// ***********************
// 82 - POSITION_BLE_REPORT_TYPE
// ***********************

pub static POSITION_BLE_REPORT_TYPE: PrmDatDistinct = PrmDatDistinct {
    id: 0x52,
    name: "position_ble_report_type",
    label: "BLE Report Type - Tracking",
    description: "BLE report type for Position messages.",
    default_val: PosBleReportTypeOptions::MAC_ADDR.val,
    distinct_vals: &[
        PosBleReportTypeOptions::MAC_ADDR,
        PosBleReportTypeOptions::SHORT_ID,
        PosBleReportTypeOptions::LONG_ID,
        PosBleReportTypeOptions::SHORT_ID_MAJOR_MINOR,
    ],
};
pub struct PosBleReportTypeOptions;
impl PosBleReportTypeOptions {
    pub const MAC_ADDR: DistinctVal = DistinctVal {
        val: 0,
        txt: "MAC Address",
    };
    pub const SHORT_ID: DistinctVal = DistinctVal {
        val: 1,
        txt: "Short ID",
    };
    pub const LONG_ID: DistinctVal = DistinctVal {
        val: 2,
        txt: "Long ID",
    };
    pub const SHORT_ID_MAJOR_MINOR: DistinctVal = DistinctVal {
        val: 3,
        txt: "Short ID with iBeacon Major/Minor",
    };
}


// ***
// *** BLE communication parameters ***
// ***


// ***********************
// 111 - BLE_CNX_ADV_DURATION
// ***********************

pub static BLE_CNX_ADV_DURATION: PrmDatDec = PrmDatDec {
    id: 0x6F,
    name: "ble_cnx_adv_duration",
    label: "BLE advertisement duration",
    description: "BLE advertisement duration [30..18000][s]",
    default_val: 600,
    range: (30, 18000),
};


// ****************************************
// 248 - BLE_BOND_INFO - SPECIAL
// ****************************************

// ble_bond_info	0xF8		0,1	0: Delete BLE bond
// 1: Indicates that the BLE bond is present on the tracker.
// Only value 0 can be set for this parameter.


// ****************************************
// 245 - BLE_CLI_ACTIVE - SPECIAL
// ****************************************

// ble_cli_active(6)	0xF5		0,1	0: Disable the CLI traces over BLE interface with tracker connected to Abeeway tracking app.
// 1: Enables the CLI traces over BLE interface with tracker connected to Abeeway tracking app.



// *************************************************
// ***                                           ***
// ***   Miscellaneous parameters                ***
// ***                                           ***
// *************************************************


// ***********************
// 13 - CONFIG_FLAGS
// ***********************

pub static CONFIG_FLAGS: PrmDatBitmap = PrmDatBitmap {
    id: 0x0d,
    name: "config_flags",
    label: "Configuration Flags",
    description: "Device Configuration Flags",
    default_val: 213055,
    bits: &[
        ConfigFlagsBit::FRAME_PENDING_ENABLED,
        ConfigFlagsBit::LONG_BUTTON_PRESS_FOR_OFF,
        ConfigFlagsBit::DEPRECATED_2,
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
        ConfigFlagsBit::RESERVED_19,
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
        ena: true,
    };
    pub const LONG_BUTTON_PRESS_FOR_OFF: BitmapBit = BitmapBit {
        bit: 1,
        txt: "Long button press activates OFF mode",
        ena: true,
    };
    pub const DEPRECATED_2: BitmapBit = BitmapBit{
        bit: 2, 
        txt: "Deprecated",
        ena: false,
    };
    pub const CONF_DL_ACK: BitmapBit = BitmapBit {
        bit: 3,
        txt: "Config downlinks are acknowledged",
        ena: true,
    };
    pub const WIFI_SCAN_ENC: BitmapBit = BitmapBit {
        bit: 4,
        txt: "WiFi scan payload is encrypted",
        ena: true,
    };
    pub const BLE_ACTIVE_AT_START: BitmapBit = BitmapBit {
        bit: 5,
        txt: "BLE connectivity interface is active at start",
        ena: true,
    };
    pub const WIFI_SCAN_ON_GEOLOC_START: BitmapBit = BitmapBit {
        bit: 6,
        txt: "Wi-Fi scan when the GNSS geoloc procdedure starts",
        ena: true,
    };
    pub const LED_BLINKS_ON_GPS_FIX: BitmapBit = BitmapBit {
        bit: 7,
        txt: "LED blinks when a GPS fix is received",
        ena: true,
    };
    pub const MOTION_START_EVENT_ENABLED: BitmapBit = BitmapBit {
        bit: 8,
        txt: "Motion Start event messages are enabled",
        ena: true,
    };
    pub const MOTION_END_EVENT_ENABLED: BitmapBit = BitmapBit {
        bit: 9,
        txt: "Motion End event messages are enabled",
        ena: true,
    };
    pub const JOIN_REQ_ON_LEAVING_OFF: BitmapBit = BitmapBit {
        bit: 10,
        txt: "A new Join Request is sent upon leaving OFF mode",
        ena: true,
    };
    pub const ASYMM_BLE_PAIRING_REJECTED: BitmapBit = BitmapBit {
        bit: 11,
        txt: "Asymmetric BLE pairing is rejected",
        ena: true,
    };
    pub const LONG_WIFI_PAYLOAD: BitmapBit = BitmapBit {
        bit: 12,
        txt: "Long Wi-Fi payload with up to 12 BSSID is enabled",
        ena: true,
    };
    pub const COLLECTION_OF_LONG_REPORT: BitmapBit = BitmapBit {
        bit: 13,
        txt: "Collection of Long Report enabled",
        ena: true,
    };
    pub const AUTOSTART_ON_LEAVING_SHIPPING_STATE: BitmapBit = BitmapBit {
        bit: 14,
        txt: "Autostart of the tracker when leaving shipping state",
        ena: true,
    };
    pub const OFF_MODE_FORBIDDEN: BitmapBit = BitmapBit {
        bit: 15,
        txt: "OFF mode is forbidden",
        ena: true,
    };
    pub const BEEPS_IN_SOS_MODE: BitmapBit = BitmapBit {
        bit: 16,
        txt: "Beeps are played during SOS mode",
        ena: true,
    };
    pub const ADR_ON_TOO_LONG_PAYLOAD: BitmapBit = BitmapBit {
        bit: 17,
        txt: "ADR is enabled in case of too long payloads",
        ena: true,
    };
    pub const EXTENDED_POSITION_PAYLOAD: BitmapBit = BitmapBit {
        bit: 18,
        txt: "Extended position payload enabled",
        ena: true,
    };
    pub const RESERVED_19: BitmapBit = BitmapBit {
        bit: 19, 
        txt: "Reserved",
        ena: false,
    };
    pub const CLI_OVER_BLE_ENABLED: BitmapBit = BitmapBit {
        bit: 20,
        txt: "CLI over BLE is enabled",
        ena: true,
    };
    pub const BLE_PASSKEY_ENABLED: BitmapBit = BitmapBit {
        bit: 21,
        txt: "BLE passkey authentication is enabled",
        ena: true,
    };
    pub const GNSS_FIX_AFTER_WIFI_SCAN_WHEN_STATIC: BitmapBit = BitmapBit {
        bit: 22,
        txt: "GNSS fix after WiFi scan when static",
        ena: true,
    };
    pub const WIFI_BEACONING_ENABLED: BitmapBit = BitmapBit {
        bit: 23,
        txt: "Wi-Fi beaconing enabled",
        ena: true,
    };
    pub const CONTINUOUS_GNSS_OPERATION: BitmapBit = BitmapBit {
        bit: 24,
        txt: "Continuous GNSS operation enabled",
        ena: true,
    };
}


// ***********************
// 40 - BATTERY_CAPACITY - TODO: CUSTOMIZATION REQUIRED!
// ***********************

// battery_capacity(11)	0x28	mAh	-1, 0, 1 - 65535	Battery setting:
//  	 	 	 	-1: Use provisioned value.
//  	 	 	 	0: Rechargeable battery.
//  	 	 	 	1-65535: Capacity of the primary battery

pub static BATTERY_CAPACITY: PrmDatBatteryCapacity = PrmDatBatteryCapacity {
    id: 0x28,
    name: "battery_capacity",
    label: "Battery Type",
    description: "Battery type setting. [-1..65535][mAh]",
    range: (-1, 65535),
    default_val: -1,
    distinct_vals: &BatteryCapacityCalcOptions::ALL_OPTIONS,
};
pub struct BatteryCapacityCalcOptions;
impl BatteryCapacityCalcOptions {
    pub const PROVISIONED: DistinctVal = DistinctVal {
        val: -1,
        txt: "Provisioned battery type",
    };
    pub const RECHARGEABLE: DistinctVal = DistinctVal {
        val: 0,
        txt: "Rechargeable battery",
    };
    pub const PRIMARY: DistinctVal = DistinctVal {
        val: 1,
        txt: "Primary battery with given capacity",
    };
    pub const ALL_OPTIONS: &'static [DistinctVal] = &[
        Self::PROVISIONED,
        Self::RECHARGEABLE,
        Self::PRIMARY,
    ]; 
}


// ***********************
// 41 - REED_SWITCH_CONFIGURATION
// ***********************

pub static REED_SWITCH_CONFIGURATION: PrmDatDistinct = PrmDatDistinct {
    id: 0x29,
    name: "reed_switch_configuration",
    label: "Reed Switch Configuration.",
    description: "Reed Switch Configuration.",
    default_val: ReedSwitchConfOptions::REED_SWITCH_DISABLED.val,
    distinct_vals: &[
        ReedSwitchConfOptions::REED_SWITCH_DISABLED,
        ReedSwitchConfOptions::RESET_DEVICE,
        ReedSwitchConfOptions::BEHAVE_AS_BUTTON,
        ReedSwitchConfOptions::BLE_ADVERTIZE,
    ],
};
pub struct ReedSwitchConfOptions;
impl ReedSwitchConfOptions {

    pub const REED_SWITCH_DISABLED: DistinctVal = DistinctVal {
        val: 0,
        txt: "Reed Switch is disabled.",
    };
    pub const RESET_DEVICE: DistinctVal = DistinctVal {
        val: 1,
        txt: "A special Reed Switch sequence causes Reset.",
    };
    pub const BEHAVE_AS_BUTTON: DistinctVal = DistinctVal {
        val: 2,
        txt: "Reed Switch behaves as the button",
    };
    pub const BLE_ADVERTIZE: DistinctVal = DistinctVal {
        val: 3,
        txt: "Start BLE advertising on Reed Switch sequence.",
    };
}


// ***********************
// 83 - BUTTON_MAPPING - TODO: CUSTOMIZATION REQUIRED!
// ***********************

// button_mapping	0x77	none	0 – 0x00086666	Configure the button action
//  	 	 	 	Bit0-3: Button long press action
//  	 	 	 	Bit4-7: Button short press action
//  	 	 	 	Bit8-11: 2 short button presses action
//  	 	 	 	Bit12-15: 3 or more short button presses action
//  	 	 	 	Bit16-19: Button long press duration in seconds, range is [1:8](13)
//  	 	 	 	Possible actions are listed below, coded on 4 bits:
//  	 	 	 	0. No action
//  	 	 	 	1. Battery level is shown with the LEDs.
//  	 	 	 	2. Start/Stop SOS.
//  	 	 	 	3. Alert.
//  	 	 	 	4. No action.
//  	 	 	 	5. Angle detection manual trigger.
//  	 	 	 	6. Special sequence activation.

pub static BUTTON_MAPPING: PrmDatButtonMapping = PrmDatButtonMapping {
    id: 0x77,
    name: "button_mapping",
    label: "Button Mapping",
    description: "Button Mapping.",
    default_val: 0x00012010,
    long_press_duration_range: (1, 8),
    action_distinct_vals: ButtonMappingOptions::ALL_OPTIONS,
};
pub struct ButtonMappingOptions;
impl ButtonMappingOptions {
    pub const NONE: DistinctVal = DistinctVal {
        val: 0,
        txt: "No action.",
    };
    pub const SHOW_BATTERY_LEVEL_BY_LED: DistinctVal = DistinctVal {
        val: 1,
        txt: "Show battery level by LED.",
    };
    pub const START_STOP_SOS: DistinctVal = DistinctVal {
        val: 2,
        txt: "Start/Stop SOS.",
    };
    pub const SEND_POSITION_ALERT: DistinctVal = DistinctVal {
        val: 3,
        txt: "Send a Position alert.",
    };
    // pub const DEPRECATED_4: DistinctVal = DistinctVal {
    //     val: 4,
    //     txt: "Deprrecated 4.",
    // };
    pub const SEND_ANGLE_DETECTION_REPORT: DistinctVal = DistinctVal {
        val: 5,
        txt: "Send an Angle Detection report.",
    };
    pub const ACTIVATE_SPECIAL_SEQUENCE: DistinctVal = DistinctVal {
        val: 6,
        txt: "Activate the functions of Special Button Sequences.",
    };
    pub const ALL_OPTIONS: &'static [DistinctVal] = &[
        Self::NONE,
        Self::SHOW_BATTERY_LEVEL_BY_LED,
        Self::START_STOP_SOS,
        Self::SEND_POSITION_ALERT,
        // Self::DEPRECATED_4,
        Self::SEND_ANGLE_DETECTION_REPORT,
        Self::ACTIVATE_SPECIAL_SEQUENCE,
    ];
}


// ***********************
// 83 - BUZZER_VOLUME
// ***********************

pub static BUZZER_VOLUME: PrmDatDec = PrmDatDec {
    id: 0x53,
    name: "buzzer_volume",
    label: "Buzzer Volume [0..100][%]",
    description: "Buzzer Volume",
    default_val: 10,
    range: (0, 100),
};


// ***********************
// 102 - PASSWORD
// ***********************

pub static PASSWORD: PrmDatDec = PrmDatDec {
    id: 0x66,
    name: "password",
    label: "CLI Password",
    description: "The password used to access CLI interface with tracker connected to USB port. [100..999999][]",
    default_val: 123,
    range: (100, 999999),
};


// ****************************************
// 247 - CONSUMPTION - SPECIAL
// ****************************************
// consumption(14)	0xF7	mAh	0, 1 –0xFFFFFFFF	0: the current consumption of the tracker is reset. A non-zero value can be used to set the current consumption of the tracker.


// ***********************
// 2 - PW_STAT_PERIOD
// ***********************

pub static PW_STAT_PERIOD: PrmDatOptional = PrmDatOptional {
    id: 0x02,
    name: "pw_stat_period",
    label: "Power Statistics Period",
    description: "Power statistics period [300..604800][s]",
    disabled_val: 0,
    default_val: 0,
    range: (300, 604800),
};


// ****************************************
// 253 - BLE_VERSION - SPECIAL - READ ONLY
// ****************************************
// ble_version	0xFD	NA	NA	BLE Firmware version. It is read only parameter.


// ****************************************
// 254 - MCU_VERSION - SPECIAL - READ ONLY
// ****************************************
// firmware_version	0xFE	NA	NA	MCU Firmware version. It is read only parameter.




// *************************************************
// ***                                           ***
// ***   Accellerometer parameters               ***
// ***                                           ***
// *************************************************


// ***********************
// 20 - MOTION_SENSITIVITY - TODO: CUSTOMIZATION REQUIRED!
// ***********************

// motion_sensitivity 0x14 0x01–0x000FFFFF	
// Accelerometer configuration. Bit fields composed by 3 octets:
        // Octet 0 (LSB). Configure the sensitivity
        //     1-30: The threshold is coded as follow: motion_sensitivity * 0.063g
        //     31-99: Same mode than above with the value 30. The threshold is capped to 30 * 0.063 = 1,89g
        //     100: Default mode (was 0 in firmware version 2.2-x and below).
        //     101-200: Default mode (sensitivity ranging from 1% to 100% as in firmware version 2.2-x and below).
        // Octet 1. Configure the Output Data Rate (ODR)
        //     0 : 12.5Hz
        //     1 : 25Hz
        //     2 : 50Hz
        //     3 : 100Hz
        //     4 : 200Hz
        // Octet 2. Full scale  selection
        //     0: 2G
        //     1: 4G
        //     2: 8G
        //     3: 16G

pub static MOTION_SENSITIVITY: PrmDatMotionSensitivity = PrmDatMotionSensitivity {
    id: 0x14,
    name: "motion_sensitivity",
    label: "Motion Sensitivity",
    description: "Motion Sensitivity. [63..1890][mg]",
    default_val: 131073,
    range_sensitivity: (1, 200),
    distinct_vals_odr: &MotionSensOdrOptions::ALL_OPTIONS,
    distinct_vals_fullscale: &MotionSensFsOptions::ALL_OPTIONS,
};

pub struct MotionSensOdrOptions;
impl MotionSensOdrOptions {
    pub const ODR_12_5_HZ: DistinctVal = DistinctVal {
        val: 0,
        txt: "12.5Hz",
    };
    pub const ODR_25_HZ: DistinctVal = DistinctVal {
        val: 1,
        txt: "25Hz",
    };
    pub const ODR_50_HZ: DistinctVal = DistinctVal {
        val: 2,
        txt: "50Hz",
    };
    pub const ODR_100_HZ: DistinctVal = DistinctVal {
        val: 3,
        txt: "100Hz",
    };
    pub const ODR_200_HZ: DistinctVal = DistinctVal {
        val: 4,
        txt: "200Hz",
    };
    pub const ALL_OPTIONS: &'static [DistinctVal] =  &[
        Self::ODR_12_5_HZ,
        Self::ODR_25_HZ,
        Self::ODR_50_HZ,
        Self::ODR_100_HZ,
        Self::ODR_200_HZ,
    ];
}

pub struct MotionSensFsOptions;
impl MotionSensFsOptions {
    pub const FS_2G: DistinctVal = DistinctVal {
        val: 0,
        txt: "2g",
    };
    pub const FS_4G: DistinctVal = DistinctVal {
        val: 1,
        txt: "4g",
    };
    pub const FS_8G: DistinctVal = DistinctVal {
        val: 2,
        txt: "8g",
    };
    pub const FS_16G: DistinctVal = DistinctVal {
        val: 3,
        txt: "16g",
    };
    pub const ALL_OPTIONS: &'static [DistinctVal] =  &[
        MotionSensFsOptions::FS_2G,
        MotionSensFsOptions::FS_4G,
        MotionSensFsOptions::FS_8G,
        MotionSensFsOptions::FS_16G,
    ];
}


// ***********************
// 21 - SHOCK_DETECTION
// ***********************

pub static SHOCK_DETECTION: PrmDatOptional = PrmDatOptional {
    id: 0x15,
    name: "shock_detection",
    label: "Shock Detection Sensitivity",
    description: "Shock Detection Sensitivity. Range: [1..255][]",
    disabled_val: 0,
    default_val: 0,
    range: (1, 255),
};


// ***********************
// 118 - MOTION_DEBOUNCE
// ***********************

pub static MOTION_DEBOUNCE: PrmDatDec = PrmDatDec {
    id: 0x76,
    name: "motion_debounce",
    label: "Motion Debounce Time",
    description: "Minimum duration of detectable movement. [0..255][20ms]",
    default_val: 1,
    range: (0, 255),
};


// ***********************
// 8 - MOTION_NB_POS
// ***********************

pub static MOTION_NB_POS: PrmDatDec = PrmDatDec {
    id: 0x08,
    name: "motion_nb_pos",
    label: "Messages on Motion Start/End",
    description: "Number of location updates sent upon motion start/end. [0..20][]",
    default_val: 1,
    range: (0, 20),
};

// ***********************
// 23 - MOTION_DURATION
// ***********************

pub static MOTION_DURATION: PrmDatDec = PrmDatDec {
    id: 0x17,
    name: "motion_duration",
    label: "Time to Detect End of Motion",
    description: "Period of time required to detect the end of a motion. [20..3600][s]",
    default_val: 180,
    range: (20, 3600),
};


// ****************************************
// 250 - ACC_X_AXIS - SPECIAL - READ ONLY
// ****************************************

// acc_x_axis	0xFA	mG	-216 – 216-1	Value measured on X axis of the accelerometer. It is read only parameter.


// ****************************************
// 250 - ACC_X_AXIS - SPECIAL - READ ONLY
// ****************************************

// acc_y_axis	0xFB	mG	-216 – 216-1	Value measured on Y axis of the accelerometer. It is read only parameter.


// ****************************************
// 250 - ACC_X_AXIS - SPECIAL - READ ONLY
// ****************************************

// acc_z_axis	0xFC	mG	-216 – 216-1	Value measured on Z axis of the accelerometer. It is read only parameter.



// *************************************************
// ***                                           ***
// ***   Temperature monitoring parameters      ***
// ***                                           ***
// *************************************************


// ***********************
// 27 - TEMPERATURE_HIGH
// ***********************

pub static TEMPERATURE_HIGH: PrmDatOptional = PrmDatOptional {
    id: 0x1B,
    name: "temperature_high",
    label: "Temperature High Threshold.",
    description: "The high threshold of Temperature. [-44..85][°C]",
    default_val: 255,
    disabled_val: 255,
    range: (-44, 85),
};


// ***********************
// 28 - TEMPERATURE_LOW
// ***********************

pub static TEMPERATURE_LOW: PrmDatOptional = PrmDatOptional {
    id: 0x1C,
    name: "temperature_low",
    label: "Temperature Low Threshold.",
    description: "The low threshold of Temperature. [-44..85][°C]",
    default_val: 255,
    disabled_val: 255,
    range: (-44, 85),
};


// ***********************
// 29 - TEMPERATURE_ACTION
// ***********************

pub static TEMPERATURE_ACTION: PrmDatDistinct = PrmDatDistinct {
    id: 0x1D,
    name: "reed_switch_configuration",
    label: "Reed Switch Configuration.",
    description: "Reed Switch Configuration.",
    default_val: TemperatureActionOptions::NO_ACTION.val,
    distinct_vals: &[
        TemperatureActionOptions::NO_ACTION,
        TemperatureActionOptions::GEOLOC_DISABLED_ON_HIGH,
        TemperatureActionOptions::GEOLOC_DISABLED_ON_LOW,
        TemperatureActionOptions::GEOLOC_DISABLED_ON_HIGH_LOW,
    ],
};
pub struct TemperatureActionOptions;
impl TemperatureActionOptions {
    pub const NO_ACTION: DistinctVal = DistinctVal {
        val: 0,
        txt: "No Actionn.",
    };
    pub const GEOLOC_DISABLED_ON_HIGH: DistinctVal = DistinctVal {
        val: 1,
        txt: "Geolocation disabled if temperature_high is reached.",
    };
    pub const GEOLOC_DISABLED_ON_LOW: DistinctVal = DistinctVal {
        val: 2,
        txt: "Geolocation disabled if temperature_low is reached.",
    };
    pub const GEOLOC_DISABLED_ON_HIGH_LOW: DistinctVal = DistinctVal {
        val: 3,
        txt: "Geolocation disabled if temperature_low or temperature_high is reached.",
    };

}



// *************************************************
// ***                                           ***
// ***   Orientation Detection parameters        ***
// ***                                           ***
// *************************************************


// ***********************
// 84 - ANGLE_DETECT_MODE
// ***********************

pub static ANGLE_DETECT_MODE: PrmDatDistinct = PrmDatDistinct {
    id: 0x54,
    name: "angle_detect_mode",
    label: "Angle Detection Mode",
    description: "Angle Detection Mode.",
    default_val: AngleDetectModeOptions::DISABLED.val,
    distinct_vals: &[
        AngleDetectModeOptions::DISABLED,
        AngleDetectModeOptions::CRITICAL_ANGLE,
        AngleDetectModeOptions::CRITICAL_ANGLE_AND_DEVIATION,
        AngleDetectModeOptions::CRITICAL_ANGLE_ON_SHOCK,
    ],
};
pub struct AngleDetectModeOptions;
impl AngleDetectModeOptions {
    pub const DISABLED: DistinctVal = DistinctVal {
        val: 0,
        txt: "Disabled",
    };
    pub const CRITICAL_ANGLE: DistinctVal = DistinctVal {
        val: 1,
        txt: "Critical angle detection.",
    };
    pub const CRITICAL_ANGLE_AND_DEVIATION: DistinctVal = DistinctVal {
        val: 2,
        txt: "Critical angle detection + Angle deviation detection.",
    };
    pub const CRITICAL_ANGLE_ON_SHOCK: DistinctVal = DistinctVal {
        val: 3,
        txt: "Critical angle detection triggered on shock detection.",
    };

}


// ***********************
// 85 - ANGLE_REF_ACQ
// ***********************

pub static ANGLE_REF_ACQ: PrmDatDistinct = PrmDatDistinct {
    id: 0x55,
    name: "angle_ref_acq",
    label: "Acquisition mode of Ref Angle",
    description: "Acquisition mode of Reference Angle.",
    default_val: AngleRefAckOptions::MANUAL.val,
    distinct_vals: &[
        AngleRefAckOptions::MANUAL,
        AngleRefAckOptions::CONFIGURED,
        AngleRefAckOptions::AUTOMATIC,
        AngleRefAckOptions::ASSISTED,
    ],
};
pub struct AngleRefAckOptions;
impl AngleRefAckOptions {
    pub const MANUAL: DistinctVal = DistinctVal {
        val: 0,
        txt: "Manual",
    };
    pub const CONFIGURED: DistinctVal = DistinctVal {
        val: 1,
        txt: "Configured",
    };
    pub const AUTOMATIC: DistinctVal = DistinctVal {
        val: 2,
        txt: "Automatic",
    };
    pub const ASSISTED: DistinctVal = DistinctVal {
        val: 3,
        txt: "Assisted",
    };

}


// ***********************
// 86 - ANGLE_REF_ACC_X
// ***********************

pub static ANGLE_REF_ACC_X: PrmDatOptional = PrmDatOptional {
    id: 0x56,
    name: "angle_ref_acc_x",
    label: "Ref Vector X",
    description: "X component of reference orientation vector. [-1000..1000][mg]",
    default_val: 0,
    disabled_val: 0xFFFF, 
    range: (-1000, 1000),
};


// ***********************
// 87 - ANGLE_REF_ACC_Y
// ***********************

pub static ANGLE_REF_ACC_Y: PrmDatOptional = PrmDatOptional {
    id: 0x57,
    name: "angle_ref_acc_y",
    label: "Ref Vector Y",
    description: "Y component of reference orientation vector. [-1000..1000][mg]",
    default_val: 0,
    disabled_val: 0xFFFF, 
    range: (-1000, 1000),
};


// ***********************
// 88 - ANGLE_REF_ACC_Z
// ***********************

pub static ANGLE_REF_ACC_Z: PrmDatOptional = PrmDatOptional {
    id: 0x58,
    name: "angle_ref_acc_z",
    label: "Ref Vector Z",
    description: "Z component of reference orientation vector. [-1000..1000][mg]",
    default_val: 0,
    disabled_val: 0xFFFF, 
    range: (-1000, 1000),
};


// ***********************
// 89 - ANGLE_CRITICAL
// ***********************

pub static ANGLE_CRITICAL: PrmDatDec = PrmDatDec {
    id: 0x59,
    name: "angle_critical",
    label: "Critical Angle",
    description: "Critical angle. [5..175][degrees]",
    default_val: 30,
    range: (5, 175),
};


// ***********************
// 90 - ANGLE_CRITICAL_HYST
// ***********************

pub static ANGLE_CRITICAL_HYST: PrmDatDec = PrmDatDec {
    id: 0x5A,
    name: "angle_critical_hyst",
    label: "Critical Angle Hysteresis",
    description: "Critical angle hysteresis. [0..180][degrees]",
    default_val: 5,
    range: (0, 180),
};


// ***********************
// 91 - ANGLE_REPORT_MODE
// ***********************

pub static ANGLE_REPORT_MODE: PrmDatBitmap = PrmDatBitmap {
    id: 0x5B,
    name: "angle_report_mode",
    label: "Angle Events to Report",
    description: "Angle Detection events to report.",
    default_val: 1,
    bits: &[
        AngleRefAckBit::NORMAL_CRITICAL,
        AngleRefAckBit::CRITICAL_NORMAL,
        AngleRefAckBit::LEARNING_NORMAL,
        AngleRefAckBit::NORMAL_LEARNING,
        AngleRefAckBit::CRITICAL_LEARNING,
    ],
};
pub struct AngleRefAckBit;
impl AngleRefAckBit {
    pub const NORMAL_CRITICAL: BitmapBit = BitmapBit {
        bit: 0,
        txt: "normal → critical",
        ena: true,
    };
    pub const CRITICAL_NORMAL: BitmapBit = BitmapBit {
        bit: 1,
        txt: "critical → normal",
        ena: true,
    };
    pub const LEARNING_NORMAL: BitmapBit = BitmapBit {
        bit: 2,
        txt: "learning → normal",
        ena: true,
    };
    pub const NORMAL_LEARNING: BitmapBit = BitmapBit {
        bit: 3,
        txt: "normal → learning",
        ena: true,
    };
    pub const CRITICAL_LEARNING: BitmapBit = BitmapBit {
        bit: 4,
        txt: "critical → learning",
        ena: true,
    };
}


// ***********************
// 92 - ANGLE_REPORT_PERIOD
// ***********************

pub static ANGLE_REPORT_PERIOD: PrmDatDec = PrmDatDec {
    id: 0x5C,
    name: "angle_report_period",
    label: "Angle Report Period",
    description: "The period between repeated event messages. 0 means that reports will be transmitted right after position messages. [60..36000][s]",
    default_val: 300,
    range: (60, 36000),
};


// ***********************
// 93 - ANGLE_REPORT_REPEAT
// ***********************

pub static ANGLE_REPORT_REPEAT: PrmDatDec = PrmDatDec {
    id: 0x5D,
    name: "angle_report_repeat",
    label: "Angle Report Repetitions",
    description: "Numbeer of Angle Report repetitions. 0 indicates that only one event message is sent (no repetition). [0..7][]",
    default_val: 0,
    range: (0, 7),
};


// ***********************
// 94 - ANGLE_RISING_TIME
// ***********************

pub static ANGLE_RISING_TIME: PrmDatDec = PrmDatDec {
    id: 0x5E,
    name: "angle_rising_time",
    label: "Rising Time",
    description: "Rising time phase duration. [0..3600][s]",
    default_val: 5,
    range: (0, 3600),
};


// ***********************
// 95 - ANGLE_FALLING_TIME
// ***********************

pub static ANGLE_FALLING_TIME: PrmDatDec = PrmDatDec {
    id: 0x5F,
    name: "angle_falling_time",
    label: "Falling Time",
    description: "Falling time phase duration. [0..3600][s]",
    default_val: 5,
    range: (0, 3600),
};


// ***********************
// 96 - ANGLE_LEARNING_TIME
// ***********************

pub static ANGLE_LEARNING_TIME: PrmDatDec = PrmDatDec {
    id: 0x60,
    name: "angle_learning_time",
    label: "Learning Time",
    description: "Learning time phase duration. [0..3600][s]",
    default_val: 5,
    range: (0, 3600),
};


// ***********************
// 97 - ANGLE_ACC_ACCURACY
// ***********************

pub static ANGLE_ACC_ACCURACY: PrmDatDec = PrmDatDec {
    id: 0x61,
    name: "angle_acc_accuracy",
    label: "Accuracy of Acc Measurement",
    description: "Accuracy of the measured acceleration. [0..1000][mg]",
    default_val: 100,
    range: (0, 1000),
};


// ***********************
// 98 - ANGLE_DEVIATION_DELTA
// ***********************

pub static ANGLE_DEVIATION_DELTA: PrmDatDec = PrmDatDec {
    id: 0x62,
    name: "angle_deviation_delta",
    label: "Reported Angle Deviation Delta",
    description: "At least this value of deviation from the previous reported orientation is needed to trigger an event message. Applicable only with angle deviation methods. [0..175][degrees]",
    default_val: 0,
    range: (0, 175),
};


// ***********************
// 99 - ANGLE_DEVIATION_MIN_INTERVAL
// ***********************

pub static ANGLE_DEVIATION_MIN_INTERVAL: PrmDatDec = PrmDatDec {
    id: 0x63,
    name: "angle_deviation_min_interval",
    label: "Min Angle Dev Report Interval",
    description: "No event message is sent before this delay from previous angle deviation event is elapsed. Any deviation before this delay is ignored. Applicable only with angle deviation methods. [0..1800][s]",
    default_val: 10,
    range: (0, 1800),
};


// ***********************
// 100 - ANGLE_DEVIATION_MAX_INTERVAL
// ***********************

pub static ANGLE_DEVIATION_MAX_INTERVAL: PrmDatDec = PrmDatDec {
    id: 0x64,
    name: "angle_deviation_max_interval",
    label: "Max Angle Dev Report Interval",
    description: "No event message is sent after this delay from previous angle deviation event is elapsed. Any deviation after this delay is ignored. Applicable only with angle deviation methods. [0..86400][s]",
    default_val: 0,
    range: (0, 86400),
};



// *************************************************
// ***                                           ***
// ***   BLE geozoning parameters                ***
// ***                                           ***
// *************************************************


// ***********************
// 24 - GEOFENCING_SCAN_PERIOD
// ***********************

pub static GEOFENCING_SCAN_PERIOD: PrmDatOptional = PrmDatOptional {
    id: 0x18,
    name: "geofencing_scan_period",
    label: "Geofencing Scan Period",
    description: "Geofencing Scan Period [1..300][s]",
    disabled_val: 0,
    default_val: 0,
    range: (1, 300),
};


// ***********************
// 25 - GEOFENCING_COLLECT_PERIOD
// ***********************

pub static GEOFENCING_COLLECT_PERIOD: PrmDatDec = PrmDatDec {
    id: 0x19,
    name: "geofencing_collect_period",
    label: "Geofencing Collect Period",
    description: "Geofencing Collect Period [15..3600][s]",
    default_val: 60,
    range: (15, 3600),
};


// ***********************
// 105 - GEOFENCING_SCAN_DURATION
// ***********************

pub static GEOFENCING_SCAN_DURATION: PrmDatDec = PrmDatDec {
    id: 0x69,
    name: "geofencing_scan_duration",
    label: "Geofencing Scan Duration",
    description: "Geofencing Scan Duration [370..3000][ms]",
    default_val: 370,
    range: (370, 3000),
};


// *************************************************
// ***                                           ***
// ***   BLE beaconing parameters                ***
// ***                                           ***
// *************************************************


// ***********************
// 106 - BEACONING_TYPE
// ***********************

pub static BEACONING_TYPE: PrmDatDistinct = PrmDatDistinct {
    id: 0x6A,
    name: "beaconing_type",
    label: "BLE Beaconing Type",
    description: "Beaconing advertisement type.",
    default_val: BeaconingTypeOptions::DISABLED.val,
    distinct_vals: &[
        BeaconingTypeOptions::DISABLED,
        // BeaconingTypeOptions::NOT_USED_1,
        BeaconingTypeOptions::QUUPPA,
        BeaconingTypeOptions::EDDYSTONE_UID,
        BeaconingTypeOptions::IBEACON,
        BeaconingTypeOptions::ALT_BEACON,
    ],
};
pub struct BeaconingTypeOptions;
impl BeaconingTypeOptions {
    pub const DISABLED: DistinctVal = DistinctVal {
        val: 0,
        txt: "Disabled",
    };
    // pub const NOT_USED_1: DistinctVal = DistinctVal {
    //     val: 1,
    //     txt: "Not used 1",
    // };
    pub const QUUPPA: DistinctVal = DistinctVal {
        val: 2,
        txt: "Quuppa",
    };
    pub const EDDYSTONE_UID: DistinctVal = DistinctVal {
        val: 3,
        txt: "Eddystone UID",
    };
    pub const IBEACON: DistinctVal = DistinctVal {
        val: 4,
        txt: "iBeacon",
    };
    pub const ALT_BEACON: DistinctVal = DistinctVal {
        val: 5,
        txt: "altBeacon",
    };
}


// ***********************
// 107 - BEACONING_TX_POWER
// ***********************

pub static BEACONING_TX_POWER: PrmDatDistinct = PrmDatDistinct {
    id: 0x6B,
    name: "beaconing_tx_power",
    label: "Beaconing TX Power",
    description: "Beaconing TX Power.",
    default_val: BeaconingTxPowerOptions::N0.val,
    distinct_vals: &[
        BeaconingTxPowerOptions::P4,
        BeaconingTxPowerOptions::P3,
        BeaconingTxPowerOptions::N0,
        BeaconingTxPowerOptions::N4,
        BeaconingTxPowerOptions::N8,
        BeaconingTxPowerOptions::N12,
        BeaconingTxPowerOptions::N16,
        BeaconingTxPowerOptions::N20,
        BeaconingTxPowerOptions::N40,
    ],
};
pub struct BeaconingTxPowerOptions;
impl BeaconingTxPowerOptions {
    pub const P4: DistinctVal = DistinctVal {
        val: 0,
        txt: "+4 dBm",
    };
    pub const P3: DistinctVal = DistinctVal {
        val: 1,
        txt: "+3 dBm",
    };
    pub const N0: DistinctVal = DistinctVal {
        val: 2,
        txt: "0 dBm",
    };
    pub const N4: DistinctVal = DistinctVal {
        val: 3,
        txt: "-4 dBm",
    };
    pub const N8: DistinctVal = DistinctVal {
        val: 4,
        txt: "-8 dBm",
    };
    pub const N12: DistinctVal = DistinctVal {
        val: 5,
        txt: "-12 dBm",
    };
    pub const N16: DistinctVal = DistinctVal {
        val: 6,
        txt: "-16 dBm",
    };
    pub const N20: DistinctVal = DistinctVal {
        val: 7,
        txt: "-20 dBm",
    };
    pub const N40: DistinctVal = DistinctVal {
        val: 8,
        txt: "-40 dBm",
    };
}


// ***********************
// 108 - BEACONING_STATIC_INTERVAL
// ***********************

pub static BEACONING_STATIC_INTERVAL: PrmDatOptional = PrmDatOptional {
    id: 0x6C,
    name: "beaconing_static_interval",
    label: "Beaconing Interval - Static",
    description: "Beaconing Interval in static state. [100..10000][ms]",
    default_val: 0,
    disabled_val: 0, 
    range: (100, 10000),
};


// ***********************
// 109 - BEACONING_MOTION_INTERVAL
// ***********************

pub static BEACONING_MOTION_INTERVAL: PrmDatOptional = PrmDatOptional {
    id: 0x6D,
    name: "beaconing_motion_interval",
    label: "Beaconing Interval - Motion",
    description: "Beaconing Interval in motion. [100..10000][ms]",
    default_val: 0,
    disabled_val: 0, 
    range: (100, 10000),
};


// ***********************
// 110 - BEACONING_MOTION_DURATION
// ***********************

pub static BEACONING_MOTION_DURATION: PrmDatDec = PrmDatDec {
    id: 0x6E,
    name: "beaconing_motion_duration",
    label: "Motion Debounce Time",
    description: "Minimum Motion Duration for detecting motion in beaconing mode. [4..255][s]",
    default_val: 0,
    range: (4, 255),
};


// ***********************
// 112 - BEACON_ID_0
// ***********************

pub static BEACON_ID_0: PrmDatDec = PrmDatDec {
    id: 0x70,
    name: "beacon_id_0",
    label: "Beacon ID 0",
    description: "Beacon ID 0",
    default_val: 0,
    range: (i32::MIN, i32::MAX),
};

// ***********************
// 113 - BEACON_ID_1
// ***********************

pub static BEACON_ID_1: PrmDatDec = PrmDatDec {
    id: 0x71,
    name: "beacon_id_1",
    label: "Beacon ID 1",
    description: "Beacon ID 1",
    default_val: 0,
    range: (i32::MIN, i32::MAX),
};

// ***********************
// 114 - BEACON_ID_2
// ***********************

pub static BEACON_ID_2: PrmDatDec = PrmDatDec {
    id: 0x72,
    name: "beacon_id_2",
    label: "Beacon ID 2",
    description: "Beacon ID 2",
    default_val: 0,
    range: (i32::MIN, i32::MAX),
};

// ***********************
// 115 - BEACON_ID_3
// ***********************

pub static BEACON_ID_3: PrmDatDec = PrmDatDec {
    id: 0x73,
    name: "beacon_id_3",
    label: "Beacon ID 3",
    description: "Beacon ID 3",
    default_val: 0,
    range: (i32::MIN, i32::MAX),
};

// ***********************
// 116 - BEACON_ID_4
// ***********************

pub static BEACON_ID_4: PrmDatDec = PrmDatDec {
    id: 0x74,
    name: "beacon_id_4",
    label: "Beacon ID 4",
    description: "Beacon ID 4",
    default_val: 0,
    range: (i32::MIN, i32::MAX),
};

