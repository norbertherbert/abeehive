
pub mod tracking_gps;
pub use tracking_gps::TEMP_TRACKING_GPS;

pub mod tracking_wifi_gps;
pub use tracking_wifi_gps::TEMP_TRACKING_WIFI_GPS;

pub mod tracking_ble_gps;
pub use tracking_ble_gps::TEMP_TRACKING_BLE_GPS;

pub mod tracking_ble_gps_realtime;
pub use tracking_ble_gps_realtime::TEMP_TRACKING_BLE_GPS_REALTIME;



pub mod ble_geozoning;
pub use ble_geozoning::TEMP_BLE_GEOZONING;

pub mod ble_scan_collection;
pub use ble_scan_collection::TEMP_BLE_SCAN_COLLECTION;

pub mod angle_detection;
pub use angle_detection::TEMP_ANGLE_DETECTION;

pub mod ble_beaconing;
pub use ble_beaconing::TEMP_BLE_BEACONING;




#[derive(Debug, PartialEq, Clone)]
pub enum CfgTemplate {
    TrackingGps,
    TrackingWifiGps,
    TrackingBleGps,
    TrackingBleGpsRealtime,
    BleGeozoning,
    BleScanCollection,
    AngleDetection,
    BleBeaconing,
}
impl CfgTemplate {
    pub fn txt (&self) -> &'static str {
        match self {
            Self::TrackingGps => TEMP_TRACKING_GPS,
            Self::TrackingWifiGps => TEMP_TRACKING_WIFI_GPS,
            Self::TrackingBleGps => TEMP_TRACKING_BLE_GPS,
            Self::TrackingBleGpsRealtime => TEMP_TRACKING_BLE_GPS_REALTIME,
            Self::BleGeozoning => TEMP_BLE_GEOZONING,
            Self::BleScanCollection => TEMP_BLE_SCAN_COLLECTION,
            Self::AngleDetection => TEMP_ANGLE_DETECTION,
            Self::BleBeaconing => TEMP_BLE_BEACONING,
        }
    }
    pub fn name (&self) -> &'static str {
        match self {
            Self::TrackingGps => "Tracking GPS",
            Self::TrackingWifiGps => "Tracking WiFi-GPS",
            Self::TrackingBleGps => "Tracking BLE-GPS",
            Self::TrackingBleGpsRealtime => "Tracking BLE-GPS - 15s",
            Self::BleGeozoning => "BLE Geozoning",
            Self::BleScanCollection => "BLE Scan Collection",
            Self::AngleDetection => "Angle Detection",
            Self::BleBeaconing => "BLE Beaconing",
        }
    }
}
