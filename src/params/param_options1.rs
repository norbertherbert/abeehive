use std::fmt;

use super::ParamType;


pub trait ParamOption {
    fn as_value(&self) -> ParamType;
}

#[repr(u8)]
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ModeOption1 {
    Standby = 0,
    MotionTracking = 1,
    PermanentTracking = 2,
    MotionStartEndTracking = 3,
    ActivityTracking = 4,
    Off = 5,
}
impl std::convert::From<ParamType> for ModeOption1 {
    fn from(value: ParamType) -> Self {
        match value {
            0 => Self::Standby,
            1 => Self::MotionTracking,
            2 => Self::PermanentTracking,
            3 => Self::MotionStartEndTracking,
            4 => Self::ActivityTracking,
            5 => Self::Off,
            _ => Self::MotionTracking, // TODO!!
        }
    }
}
impl fmt::Display for ModeOption1 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        let item_text = match self {
            Self::Standby => "Standby",
            Self::MotionTracking => "Motion tracking",
            Self::PermanentTracking => "Permanent tracking",
            Self::MotionStartEndTracking => "Motion start/end tracking",
            Self::ActivityTracking => "Activity tracking",
            Self::Off => "Off",
        };

        write!(f, "{}", item_text)

    }
}
impl ParamOption for ModeOption1 {
    fn as_value(&self) -> ParamType { 
        (unsafe { *(self as *const Self) as u8 }) as ParamType 
    }
}
impl ModeOption1 {
    pub const VARIANTS: &'static [Self] = &[
        Self::Standby, Self::MotionTracking, Self::PermanentTracking, Self::MotionStartEndTracking, Self::ActivityTracking, Self::Off
    ];
}

#[repr(u8)]
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum GeolocSensorOption1 {
    Wifi = 0,
    Gps = 1,
    Lpgps = 2,
    // Reserved3 = 3,
    // Reserved4 = 4,
    // Reserved5 = 5,
    WifiGps = 6,
    WifiLpgps = 7,
    // Reserved8 = 8,
    WifiLpgpsWifiGps = 9,
    Ble = 10,
    BleGps = 11,
    BleLpgps = 12,
}
impl std::convert::From<ParamType> for GeolocSensorOption1 {
    fn from(value: ParamType) -> Self {
        match value {
            0 => Self::Wifi,
            1 => Self::Gps,
            2 => Self::Lpgps,
            // 3 => Self::Reserved3,
            // 4 => Self::Reserved4,
            // 5 => Self::Reserved5,
            6 => Self::WifiGps,
            7 => Self::WifiLpgps,
            // 8 => Self::Reserved8,
            9 => Self::WifiLpgpsWifiGps,
            10 => Self::Ble,
            11 => Self::BleGps,
            12 => Self::BleLpgps,
            _ => Self::Gps, // TODO!!
        }
    }
}
impl fmt::Display for GeolocSensorOption1 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        let item_text = match self {
            Self::Wifi => "WiFi",
            Self::Gps => "GPS",
            Self::Lpgps => "LPGPS",
            // Self::Reserved3 => "Reserved 3",
            // Self::Reserved4 => "Reserved 4",
            // Self::Reserved5 => "Reserved 5",
            Self::WifiGps => "WiFi-GPS",
            Self::WifiLpgps => "WiFi-LPGPS",
            // Self::Reserved8 => "Reserved 8",
            Self::WifiLpgpsWifiGps => "WiFi-LPGPS/WiFi-GPS",
            Self::Ble => "BLE",
            Self::BleGps => "BLE-GPS",
            Self::BleLpgps => "BLE-LPGPS",
        };

        write!(f, "{}", item_text)

    }
}
impl ParamOption for GeolocSensorOption1 {
    fn as_value(&self) -> ParamType {
        (unsafe { *(self as *const Self) as u8 }) as ParamType
    }
}
impl GeolocSensorOption1 {
    pub const VARIANTS: &'static [Self] = &[
        Self::Wifi, Self::Gps, Self::Lpgps, 
        // Self::Reserved3, Self::Reserved4, Self::Reserved5,
        Self::WifiGps, Self::WifiLpgps, 
        // Self::Reserved8,
        Self::WifiLpgpsWifiGps, Self::Ble, Self::BleGps, Self::BleLpgps,
    ];

}

#[repr(u8)]
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum GeolocMethodOption1 {
    Wifi = 0,
    Gps = 1,
    Lpgps = 2,
    WifiGps = 3,
    WifiLpgps = 4,
    Ble = 5,
    BleGps = 6,
    BleLpgps = 7,
}
impl std::convert::From<ParamType> for GeolocMethodOption1 {
    fn from(value: ParamType) -> Self {
        match value {
            0 => Self::Wifi,
            1 => Self::Gps,
            2 => Self::Lpgps,
            3 => Self::WifiGps,
            4 => Self::WifiLpgps,
            5 => Self::Ble,
            6 => Self::BleGps,
            7 => Self::BleLpgps,
            _ => Self::Gps, // TODO!!
        }
    }
}
impl fmt::Display for GeolocMethodOption1 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        let item_text = match self {
            Self::Wifi => "WiFi",
            Self::Gps => "GPS",
            Self::Lpgps => "LPGPS",
            Self::WifiGps => "WiFi-GPS",
            Self::WifiLpgps => "WiFi-LPGPS",
            Self::Ble => "BLE",
            Self::BleGps => "BLE-GPS",
            Self::BleLpgps => "BLE-LPGPS",
        };

        write!(f, "{}", item_text)

    }
}
impl ParamOption for GeolocMethodOption1 {
    fn as_value(&self) -> ParamType {
        (unsafe { *(self as *const Self) as u8 }) as ParamType
    }
}
impl GeolocMethodOption1 {
    pub const VARIANTS: &'static [Self] = &[
        Self::Wifi, Self::Gps, Self::Lpgps, 
        Self::WifiGps, Self::WifiLpgps, 
        Self::Ble, Self::BleGps, Self::BleLpgps,
    ];
}

#[repr(u8)]
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum TransmitStratOption1 {
    SingleFixed = 0,
    SingleRandom = 1,
    DoubleRandom = 2,
    DoubleFixed = 3,
    // Reserved4 = 4,
    Custom = 5,
}
impl std::convert::From<ParamType> for TransmitStratOption1 {
    fn from(value: ParamType) -> Self {
        match value {
            0 => Self::SingleFixed,
            1 => Self::SingleRandom,
            2 => Self::DoubleRandom,
            3 => Self::DoubleFixed,
            // 4 => Self::Reserved4,
            5 => Self::Custom,
            _ => Self::DoubleRandom, // TODO!!
        }
    }
}
impl fmt::Display for TransmitStratOption1 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        let item_text = match self {
            Self::SingleFixed => "Single fixed",
            Self::SingleRandom => "Single random",
            Self::DoubleRandom => "Double random",
            Self::DoubleFixed => "Doubl fixed",
            // Self::Reserved => "Reserved",
            Self::Custom => "Custom",
        };

        write!(f, "{}", item_text)

    }
}
impl ParamOption for TransmitStratOption1 {
    fn as_value(&self) -> ParamType {
        (unsafe { *(self as *const Self) as u8 }) as ParamType
    }
}
impl TransmitStratOption1 {
    pub const VARIANTS: &'static [Self] = &[
        Self::SingleFixed, Self::SingleRandom, Self::DoubleRandom, Self::DoubleFixed, 
        // Self::Reserved, 
        Self::Custom
    ];
}

