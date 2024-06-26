use std::collections::BTreeMap;
use anyhow::{anyhow, Result};
use gloo::console::log;
use base64::prelude::*;

use crate::prm::{
    typ::PrmVal,
    idx::{ id_to_data_map, name_to_data_map },
    dat::*,
};

/// Parameter Validated Value

#[derive(Debug, Clone, PartialEq)]
pub enum PrmVVal {
    Valid(PrmVal),
    Invalid((PrmVal, String)),
    InvalidTxt((String, String))
}

/// Parameter Validated Values

#[derive(Debug, PartialEq)]
pub struct PrmVVals {
    vvals_data: BTreeMap<u8, PrmVVal>
}

impl Default for PrmVVals {
    fn default() -> Self {

        let mut map: BTreeMap<u8, PrmVVal> = BTreeMap::new();

        // map.insert(MODE.id, PrmVVal::Valid(MODE.default_val));
        // map.insert(UL_PERIOD.id, PrmVVal::Valid(UL_PERIOD.default_val));
        // map.insert(LORA_PERIOD.id, PrmVVal::Valid(LORA_PERIOD.default_val));
        // map.insert(PERIODIC_POS_PERIOD.id, PrmVVal::Valid(PERIODIC_POS_PERIOD.default_val));
        // map.insert(GEOLOC_SENSOR.id, PrmVVal::Valid(GEOLOC_SENSOR.default_val));
        // map.insert(GEOLOC_METHOD.id, PrmVVal::Valid(GEOLOC_METHOD.default_val));
        // map.insert(TRANSMIT_STRAT.id, PrmVVal::Valid(TRANSMIT_STRAT.default_val));
        // map.insert(TRANSMIT_STRAT_CUSTOM.id, PrmVVal::Valid(TRANSMIT_STRAT_CUSTOM.default_val));
        // map.insert(CONFIG_FLAGS.id, PrmVVal::Valid(CONFIG_FLAGS.default_val));
        // map.insert(MOTION_SENSITIVITY.id, PrmVVal::Valid(MOTION_SENSITIVITY.default_val));
        // map.insert(BUTTON_MAPPING.id, PrmVVal::Valid(BUTTON_MAPPING.default_val));
        // map.insert(BATTERY_CAPACITY.id, PrmVVal::Valid(BATTERY_CAPACITY.default_val));

        map.insert(UL_PERIOD.id, PrmVVal::Valid(UL_PERIOD.default_val));
        map.insert(LORA_PERIOD.id, PrmVVal::Valid(LORA_PERIOD.default_val));
        map.insert(PW_STAT_PERIOD.id, PrmVVal::Valid(PW_STAT_PERIOD.default_val));
        map.insert(PERIODIC_POS_PERIOD.id, PrmVVal::Valid(PERIODIC_POS_PERIOD.default_val));
        // map.insert(UNKNOWN.id, PrmVVal::Valid(UNKNOWN.default_val));
        map.insert(GEOLOC_SENSOR.id, PrmVVal::Valid(GEOLOC_SENSOR.default_val));
        map.insert(GEOLOC_METHOD.id, PrmVVal::Valid(GEOLOC_METHOD.default_val));
        // map.insert(ANTENNA.id, PrmVVal::Valid(ANTENNA.default_val));
        map.insert(MOTION_NB_POS.id, PrmVVal::Valid(MOTION_NB_POS.default_val));
        map.insert(GPS_TIMEOUT.id, PrmVVal::Valid(GPS_TIMEOUT.default_val));
        map.insert(AGPS_TIMEOUT.id, PrmVVal::Valid(AGPS_TIMEOUT.default_val));
        map.insert(GPS_EHPE.id, PrmVVal::Valid(GPS_EHPE.default_val));
        map.insert(GPS_CONVERGENCE.id, PrmVVal::Valid(GPS_CONVERGENCE.default_val));
        map.insert(CONFIG_FLAGS.id, PrmVVal::Valid(CONFIG_FLAGS.default_val));
        map.insert(TRANSMIT_STRAT.id, PrmVVal::Valid(TRANSMIT_STRAT.default_val));
        map.insert(BLE_BEACON_CNT.id, PrmVVal::Valid(BLE_BEACON_CNT.default_val));
        map.insert(BLE_BEACON_TIMEOUT.id, PrmVVal::Valid(BLE_BEACON_TIMEOUT.default_val));
        map.insert(GPS_STANDBY_TIMEOUT.id, PrmVVal::Valid(GPS_STANDBY_TIMEOUT.default_val));
        map.insert(CONFIRMED_UL_BITMAP.id, PrmVVal::Valid(CONFIRMED_UL_BITMAP.default_val));
        map.insert(CONFIRMED_UL_RETRY.id, PrmVVal::Valid(CONFIRMED_UL_RETRY.default_val));
        map.insert(MOTION_SENSITIVITY.id, PrmVVal::Valid(MOTION_SENSITIVITY.default_val));
        map.insert(SHOCK_DETECTION.id, PrmVVal::Valid(SHOCK_DETECTION.default_val));
        map.insert(PERIODIC_ACTIVITY_PERIOD.id, PrmVVal::Valid(PERIODIC_ACTIVITY_PERIOD.default_val));
        map.insert(MOTION_DURATION.id, PrmVVal::Valid(MOTION_DURATION.default_val));
        map.insert(GEOFENCING_SCAN_PERIOD.id, PrmVVal::Valid(GEOFENCING_SCAN_PERIOD.default_val));
        map.insert(GEOFENCING_COLLECT_PERIOD.id, PrmVVal::Valid(GEOFENCING_COLLECT_PERIOD.default_val));
        map.insert(BLE_RSSI_FILTER.id, PrmVVal::Valid(BLE_RSSI_FILTER.default_val));
        map.insert(TEMPERATURE_HIGH.id, PrmVVal::Valid(TEMPERATURE_HIGH.default_val));
        map.insert(TEMPERATURE_LOW.id, PrmVVal::Valid(TEMPERATURE_LOW.default_val));
        map.insert(TEMPERATURE_ACTION.id, PrmVVal::Valid(TEMPERATURE_ACTION.default_val));
        map.insert(TRANSMIT_STRAT_CUSTOM.id, PrmVVal::Valid(TRANSMIT_STRAT_CUSTOM.default_val));
        map.insert(NETWORK_TIMEOUT_CHECK.id, PrmVVal::Valid(NETWORK_TIMEOUT_CHECK.default_val));
        map.insert(NETWORK_TIMEOUT_RESET.id, PrmVVal::Valid(NETWORK_TIMEOUT_RESET.default_val));
        map.insert(COLLECTION_SCAN_TYPE.id, PrmVVal::Valid(COLLECTION_SCAN_TYPE.default_val));
        map.insert(COLLECTION_NB_ENTRY.id, PrmVVal::Valid(COLLECTION_NB_ENTRY.default_val));
        map.insert(COLLECTION_BLE_FILTER_TYPE.id, PrmVVal::Valid(COLLECTION_BLE_FILTER_TYPE.default_val));
        map.insert(COLLECTION_BLE_FILTER_MAIN_1.id, PrmVVal::Valid(COLLECTION_BLE_FILTER_MAIN_1.default_val));
        map.insert(COLLECTION_BLE_FILTER_MAIN_2.id, PrmVVal::Valid(COLLECTION_BLE_FILTER_MAIN_2.default_val));
        map.insert(COLLECTION_BLE_FILTER_SEC_VALUE.id, PrmVVal::Valid(COLLECTION_BLE_FILTER_SEC_VALUE.default_val));
        map.insert(COLLECTION_BLE_FILTER_SEC_MASK.id, PrmVVal::Valid(COLLECTION_BLE_FILTER_SEC_MASK.default_val));
        map.insert(BATTERY_CAPACITY.id, PrmVVal::Valid(BATTERY_CAPACITY.default_val));
        map.insert(REED_SWITCH_CONFIGURATION.id, PrmVVal::Valid(REED_SWITCH_CONFIGURATION.default_val));
        map.insert(GNSS_CONSTELLATION.id, PrmVVal::Valid(GNSS_CONSTELLATION.default_val));
        // map.insert(PROX_SCAN_PWR_MIN.id, PrmVVal::Valid(PROX_SCAN_PWR_MIN.default_val));
        // map.insert(PROX_DISTANCE_COEF.id, PrmVVal::Valid(PROX_DISTANCE_COEF.default_val));
        // map.insert(PROX_SCAN_FREQUENCY.id, PrmVVal::Valid(PROX_SCAN_FREQUENCY.default_val));
        // map.insert(PROX_BACKTRACE_MAX_AGE.id, PrmVVal::Valid(PROX_BACKTRACE_MAX_AGE.default_val));
        // map.insert(PROX_DISTANCE_SLIDING_WINDOW.id, PrmVVal::Valid(PROX_DISTANCE_SLIDING_WINDOW.default_val));
        // map.insert(PROX_EXPOSURE_50.id, PrmVVal::Valid(PROX_EXPOSURE_50.default_val));
        // map.insert(PROX_EXPOSURE_100.id, PrmVVal::Valid(PROX_EXPOSURE_100.default_val));
        // map.insert(PROX_EXPOSURE_150.id, PrmVVal::Valid(PROX_EXPOSURE_150.default_val));
        // map.insert(PROX_EXPOSURE_200.id, PrmVVal::Valid(PROX_EXPOSURE_200.default_val));
        // map.insert(PROX_EXPOSURE_250.id, PrmVVal::Valid(PROX_EXPOSURE_250.default_val));
        // map.insert(PROX_EXPOSURE_300.id, PrmVVal::Valid(PROX_EXPOSURE_300.default_val));
        // map.insert(PROX_EXPOSURE_400.id, PrmVVal::Valid(PROX_EXPOSURE_400.default_val));
        // map.insert(PROX_ALARM_DIST_IMMEDIATE.id, PrmVVal::Valid(PROX_ALARM_DIST_IMMEDIATE.default_val));
        // map.insert(PROX_ALARM_EXPOSURE.id, PrmVVal::Valid(PROX_ALARM_EXPOSURE.default_val));
        // map.insert(PROX_WARN_DIST_IMMEDIATE.id, PrmVVal::Valid(PROX_WARN_DIST_IMMEDIATE.default_val));
        // map.insert(PROX_WARN_EXPOSURE.id, PrmVVal::Valid(PROX_WARN_EXPOSURE.default_val));
        // map.insert(PROX_RECORD_DIST_IMMEDIATE.id, PrmVVal::Valid(PROX_RECORD_DIST_IMMEDIATE.default_val));
        // map.insert(PROX_RECORD_EXPOSURE.id, PrmVVal::Valid(PROX_RECORD_EXPOSURE.default_val));
        // map.insert(PROX_ALARM_BUZ_DURATION.id, PrmVVal::Valid(PROX_ALARM_BUZ_DURATION.default_val));
        // map.insert(PROX_WARN_BUZ_DURATION.id, PrmVVal::Valid(PROX_WARN_BUZ_DURATION.default_val));
        // map.insert(PROX_CONTACT_POLICY.id, PrmVVal::Valid(PROX_CONTACT_POLICY.default_val));
        // map.insert(PROX_SCAN_DURATION.id, PrmVVal::Valid(PROX_SCAN_DURATION.default_val));
        // map.insert(PROX_SCAN_WINDOW.id, PrmVVal::Valid(PROX_SCAN_WINDOW.default_val));
        // map.insert(PROX_SCAN_INTERVAL.id, PrmVVal::Valid(PROX_SCAN_INTERVAL.default_val));
        // map.insert(PROX_ALARM_REMANENCE.id, PrmVVal::Valid(PROX_ALARM_REMANENCE.default_val));
        // map.insert(PROX_WARN_REMANENCE.id, PrmVVal::Valid(PROX_WARN_REMANENCE.default_val));
        // map.insert(PROX_BCN_REPEAT.id, PrmVVal::Valid(PROX_BCN_REPEAT.default_val));
        // map.insert(PROX_BCN_TX_POWER.id, PrmVVal::Valid(PROX_BCN_TX_POWER.default_val));
        // map.insert(PROX_REMINDER_PERIOD.id, PrmVVal::Valid(PROX_REMINDER_PERIOD.default_val));
        // map.insert(PROX_REMINDER_DISTANCE.id, PrmVVal::Valid(PROX_REMINDER_DISTANCE.default_val));
        // map.insert(PROX_WARN_DISABLE_DIST.id, PrmVVal::Valid(PROX_WARN_DISABLE_DIST.default_val));
        // map.insert(PROX_ALARM_DISABLE_DIST.id, PrmVVal::Valid(PROX_ALARM_DISABLE_DIST.default_val));
        // map.insert(PROX_MAX_SPEED_FILTER.id, PrmVVal::Valid(PROX_MAX_SPEED_FILTER.default_val));
        // map.insert(PROX_MAX_UPDATE.id, PrmVVal::Valid(PROX_MAX_UPDATE.default_val));
        map.insert(POSITION_BLE_FILTER_TYPE.id, PrmVVal::Valid(POSITION_BLE_FILTER_TYPE.default_val));
        map.insert(POSITION_BLE_FILTER_MAIN_1.id, PrmVVal::Valid(POSITION_BLE_FILTER_MAIN_1.default_val));
        map.insert(POSITION_BLE_FILTER_MAIN_2.id, PrmVVal::Valid(POSITION_BLE_FILTER_MAIN_2.default_val));
        map.insert(POSITION_BLE_FILTER_SEC_VALUE.id, PrmVVal::Valid(POSITION_BLE_FILTER_SEC_VALUE.default_val));
        map.insert(POSITION_BLE_FILTER_SEC_MASK.id, PrmVVal::Valid(POSITION_BLE_FILTER_SEC_MASK.default_val));
        map.insert(POSITION_BLE_REPORT_TYPE.id, PrmVVal::Valid(POSITION_BLE_REPORT_TYPE.default_val));
        map.insert(BUZZER_VOLUME.id, PrmVVal::Valid(BUZZER_VOLUME.default_val));
        map.insert(ANGLE_DETECT_MODE.id, PrmVVal::Valid(ANGLE_DETECT_MODE.default_val));
        map.insert(ANGLE_REF_ACQ.id, PrmVVal::Valid(ANGLE_REF_ACQ.default_val));
        map.insert(ANGLE_REF_ACC_X.id, PrmVVal::Valid(ANGLE_REF_ACC_X.default_val));
        map.insert(ANGLE_REF_ACC_Y.id, PrmVVal::Valid(ANGLE_REF_ACC_Y.default_val));
        map.insert(ANGLE_REF_ACC_Z.id, PrmVVal::Valid(ANGLE_REF_ACC_Z.default_val));
        map.insert(ANGLE_CRITICAL.id, PrmVVal::Valid(ANGLE_CRITICAL.default_val));
        map.insert(ANGLE_CRITICAL_HYST.id, PrmVVal::Valid(ANGLE_CRITICAL_HYST.default_val));
        map.insert(ANGLE_REPORT_MODE.id, PrmVVal::Valid(ANGLE_REPORT_MODE.default_val));
        map.insert(ANGLE_REPORT_PERIOD.id, PrmVVal::Valid(ANGLE_REPORT_PERIOD.default_val));
        map.insert(ANGLE_REPORT_REPEAT.id, PrmVVal::Valid(ANGLE_REPORT_REPEAT.default_val));
        map.insert(ANGLE_RISING_TIME.id, PrmVVal::Valid(ANGLE_RISING_TIME.default_val));
        map.insert(ANGLE_FALLING_TIME.id, PrmVVal::Valid(ANGLE_FALLING_TIME.default_val));
        map.insert(ANGLE_LEARNING_TIME.id, PrmVVal::Valid(ANGLE_LEARNING_TIME.default_val));
        map.insert(ANGLE_ACC_ACCURACY.id, PrmVVal::Valid(ANGLE_ACC_ACCURACY.default_val));
        map.insert(ANGLE_DEVIATION_DELTA.id, PrmVVal::Valid(ANGLE_DEVIATION_DELTA.default_val));
        map.insert(ANGLE_DEVIATION_MIN_INTERVAL.id, PrmVVal::Valid(ANGLE_DEVIATION_MIN_INTERVAL.default_val));
        map.insert(ANGLE_DEVIATION_MAX_INTERVAL.id, PrmVVal::Valid(ANGLE_DEVIATION_MAX_INTERVAL.default_val));
        map.insert(DEFAULT_PROFILE.id, PrmVVal::Valid(DEFAULT_PROFILE.default_val));
        map.insert(PASSWORD.id, PrmVVal::Valid(PASSWORD.default_val));
        map.insert(GPS_T0_TIMEOUT.id, PrmVVal::Valid(GPS_T0_TIMEOUT.default_val));
        map.insert(GPS_FIX_TIMEOUT.id, PrmVVal::Valid(GPS_FIX_TIMEOUT.default_val));
        map.insert(GEOFENCING_SCAN_DURATION.id, PrmVVal::Valid(GEOFENCING_SCAN_DURATION.default_val));
        map.insert(BEACONING_TYPE.id, PrmVVal::Valid(BEACONING_TYPE.default_val));
        map.insert(BEACONING_TX_POWER.id, PrmVVal::Valid(BEACONING_TX_POWER.default_val));
        map.insert(BEACONING_STATIC_INTERVAL.id, PrmVVal::Valid(BEACONING_STATIC_INTERVAL.default_val));
        map.insert(BEACONING_MOTION_INTERVAL.id, PrmVVal::Valid(BEACONING_MOTION_INTERVAL.default_val));
        map.insert(BEACONING_MOTION_DURATION.id, PrmVVal::Valid(BEACONING_MOTION_DURATION.default_val));
        map.insert(BLE_CNX_ADV_DURATION.id, PrmVVal::Valid(BLE_CNX_ADV_DURATION.default_val));
        map.insert(BEACON_ID_0.id, PrmVVal::Valid(BEACON_ID_0.default_val));
        map.insert(BEACON_ID_1.id, PrmVVal::Valid(BEACON_ID_1.default_val));
        map.insert(BEACON_ID_2.id, PrmVVal::Valid(BEACON_ID_2.default_val));
        map.insert(BEACON_ID_3.id, PrmVVal::Valid(BEACON_ID_3.default_val));
        map.insert(BEACON_ID_4.id, PrmVVal::Valid(BEACON_ID_4.default_val));
        map.insert(SOS_PERIOD.id, PrmVVal::Valid(SOS_PERIOD.default_val));
        map.insert(MOTION_DEBOUNCE.id, PrmVVal::Valid(MOTION_DEBOUNCE.default_val));
        map.insert(BUTTON_MAPPING.id, PrmVVal::Valid(BUTTON_MAPPING.default_val));
        map.insert(DEFAULT_DATARATE.id, PrmVVal::Valid(DEFAULT_DATARATE.default_val));
        map.insert(GPS_EHPE_MOTION.id, PrmVVal::Valid(GPS_EHPE_MOTION.default_val));
        map.insert(GPS_CONVERGENCE_MOTION.id, PrmVVal::Valid(GPS_CONVERGENCE_MOTION.default_val));
        map.insert(GPS_T0_TIMEOUT_MOTION.id, PrmVVal::Valid(GPS_T0_TIMEOUT_MOTION.default_val));
        // map.insert(BLE_CLI_ACTIVE.id, PrmVVal::Valid(BLE_CLI_ACTIVE.default_val));
        map.insert(PROFILE.id, PrmVVal::Valid(PROFILE.default_val));
        // map.insert(CONSUMPTION.id, PrmVVal::Valid(CONSUMPTION.default_val));
        // map.insert(BLE_BOND_INFO.id, PrmVVal::Valid(BLE_BOND_INFO.default_val));
        map.insert(MODE.id, PrmVVal::Valid(MODE.default_val));
        // map.insert(ACC_X_AXIS.id, PrmVVal::Valid(ACC_X_AXIS.default_val));
        // map.insert(ACC_Y_AXIS.id, PrmVVal::Valid(ACC_Y_AXIS.default_val));
        // map.insert(ACC_Z_AXIS.id, PrmVVal::Valid(ACC_Z_AXIS.default_val));
        // map.insert(BLE_VERSION.id, PrmVVal::Valid(BLE_VERSION.default_val));
        // map.insert(FIRMWARE_VERSION.id, PrmVVal::Valid(FIRMWARE_VERSION.default_val));
        








        Self { vvals_data: map }

    }
}

impl PrmVVals {

    pub fn new() -> Self {
        Self{vvals_data: BTreeMap::new()}
    }

    pub fn is_empty(&self) -> bool {
        self.vvals_data.is_empty()
    }

    pub fn set_val_by_id(&mut self, id: u8, val: PrmVal) -> Result<()> {
        let Some(param_data) = id_to_data_map().get(&id) else {
            return Err(anyhow!("invalid parameter id: {}", id))
        };
        let vval = param_data.vval_from_val(val);
        self.vvals_data.insert(id, vval);
        Ok(())
    }

    pub fn set_txt_by_id(&mut self, id: u8, txt: &str) -> Result<()> {
        let Some(param_data) = id_to_data_map().get(&id) else {
            return Err(anyhow!("invalid parameter id: {}", id))
        };
        let vval = param_data.vval_from_txt(txt);
        self.vvals_data.insert(id, vval);
        Ok(())
    }

    pub fn set_val_by_name(&mut self, name: String, val: PrmVal) -> Result<()> {
        let Some(param_data) = name_to_data_map().get(&name as &str) else {
            return Err(anyhow!("invalid parameter name: {}", name))
        };
        let vval = param_data.vval_from_val(val);
        self.vvals_data.insert(param_data.id(), vval);
        Ok(())
    }

    pub fn set_txt_by_name(&mut self, name: String, txt: &str) -> Result<()> {
        let Some(param_data) = name_to_data_map().get(&name as &str) else {
            return Err(anyhow!("invalid parameter name: {}", name))
        };
        let vval = param_data.vval_from_txt(txt);
        self.vvals_data.insert(param_data.id(), vval);
        Ok(())
    }

    pub fn get_by_id(&self, id: u8) -> Result<Option<&PrmVVal>> {
        if !id_to_data_map().contains_key(&id) {
            return Err(anyhow!("invalid parameter id: {}", id));
        }
        Ok(
            self.vvals_data.get(&id)
        )
    }

    pub fn get_by_name(&self, name: &str) -> Result<Option<&PrmVVal>> {
        let Some(param_data) = name_to_data_map().get(&name as &str) else {
            return Err(anyhow!("invalid parameter name: {}", name));
        };
        Ok(
            self.vvals_data.get(&param_data.id())
        )
    }

    pub fn iter(&self) -> std::collections::btree_map::Iter<u8, PrmVVal> {
        self.vvals_data.iter()
    }

    pub fn to_cfg_string(&self) -> String {
        let mut cfg: String = "".to_owned();
        for (id, vval) in self.iter() {
            let Some(param_data) = id_to_data_map().get(id) else {
                continue
            };

            match vval {
                PrmVVal::Valid(val) => {
                    cfg += &format!("{} = {}\r\n", param_data.name(), val);
                },
                PrmVVal::Invalid((val, err_msg)) => {
                    cfg += &format!("# ERROR: {}\r\n", err_msg);
                    cfg += &format!("# {} = {}\r\n", param_data.name(), val);
                },
                PrmVVal::InvalidTxt((txt, err_msg)) => {
                    cfg += &format!("# ERROR: {}\r\n", err_msg);
                    cfg += &format!("# {} = {}\r\n", param_data.name(), txt);
                },
            }

        }
        cfg
    }

    pub fn to_lwdl_string(&self) -> String {
        let mut cfg: String =
r#"# LoRaWAN Downlink Configuration Commands must be sent to FPort=2.
# A message paylod in hex-encoded format should start with the "0b0a" prefix.
# You can compose a LoRaWAN downlink message from multiple (max 5) parameters 
# so that the prefix is used only once at the beginning of the message.

"#.to_owned();

        for (id, vval) in self.iter() {
            let Some(param_data) = id_to_data_map().get(id) else {
                continue
            };

            match vval {
                PrmVVal::Valid(val) => {

                    let val_bytes = val.to_be_bytes();
                    let pl = vec![0x0b, 0x0a, *id, val_bytes[0], val_bytes[1], val_bytes[2], val_bytes[3]];

                    cfg += &format!("{} = {}\r\n", param_data.name(), val);
                    cfg += &format!("    Payload Hex:    \"(0b0a){:02x}{:08x}\"\r\n", id, val);
                    cfg += &format!("    Payload Base64: \"{}\"\r\n\r\n", BASE64_STANDARD.encode(pl));

                },
                PrmVVal::Invalid((val, err_msg)) => {
                    cfg += &format!("# ERROR: {}\r\n", err_msg);
                    cfg += &format!("# {} = {}\r\n", param_data.name(), val);
                },
                PrmVVal::InvalidTxt((txt, err_msg)) => {
                    cfg += &format!("# ERROR: {}\r\n", err_msg);
                    cfg += &format!("# {} = {}\r\n", param_data.name(), txt);
                },
            }

        }
        cfg
    }

    pub fn to_cfg_vec(&self) -> Vec<(u8, PrmVal)> {

            let mut cfg_vec: Vec<(u8, PrmVal)> = Vec::new(); 
            for (id, vval) in self.iter() {
                if let PrmVVal::Valid(val) = vval {
                    cfg_vec.push((*id, *val))
                } else {
                    continue
                }
            }

            cfg_vec

    }

    pub fn from_cfg_str(cfg_string: &str) -> Result<Self>{
    
        // let mut vvals = Self::new();
        let mut vvals = Self::default();
    
        for line in cfg_string.lines() {
    
            // Remove comments, marked by #
            let line = line
                .split("#")
                .next()
                .expect("Cannot fail")
                .trim();
            
            if line.is_empty() {
                continue 
            }
    
            let mut words = line.split('=');
    
            if let (Some(param_name), Some(param_value)) = (words.next(), words.next()) {
    
                // If this is Some, then there were two '=' characters in the line
                if words.next().is_some() {
                    log!(format!("Invalid line was ignored while parsing the config file: '{}'", line));
                    continue;
                }
    
                // Parse parameter name
                let name = param_name.trim(); 
                let Some(param_data) = name_to_data_map().get(name) else {
                    log!(format!("Invalid parameter name was ignored while parsing the config file: '{}'", line));
                    continue;
                };
                
                let id = param_data.id();

                // // Parse parameter value
                // let Ok(val) = param_value.trim().parse::<i32>() else {
                //     log!(format!("Invalid parameter value was ignored while parsing the config file: '{}'", line);
                //     continue;
                // };
    
                // // Insert and validate the value
                // match vvals.set_by_id(id, val) {
                //     Ok(_) => {}
                //     Err(e) => {
                //         log!(format!("Parameter ignored. Error while inserting: {}", e);
                //         continue;
                //     }
                // }
                
                // Insert and validate the txt
                match vvals.set_txt_by_id(id, param_value.trim()) {
                    Ok(_) => {}
                    Err(e) => {
                        log!(format!("Parameter ignored. Error while inserting: {}", e));
                        continue;
                    }
                }

            } else {
                log!(format!("Invalid line was ignored while parsing the config file: '{}'", line));
                continue;
            }
        }
    
        Ok(vvals)
    
    }

    pub fn from_cfg_vec(cfg_vec: &Vec<(u8, PrmVal)>) -> Result<Self> {
        // let mut vvals = Self::new();
        let mut vvals = Self::default();
        for item in cfg_vec {

            match vvals.set_val_by_id(item.0, item.1) {
                Ok(_) => {}
                Err(e) => {
                    log!(format!("Parameter ignored. Error while inserting: {}", e));
                    continue;
                }
            }

        }
        Ok(vvals)
    }

    pub fn from_cfg_file(file_path: &str) -> Result<Self>{

        let cfg: String = std::fs::read_to_string(file_path)
        .map_err(|e| {
            match e.kind() {
                std::io::ErrorKind::NotFound => {
                    anyhow!("The file '{}' was not found!", file_path)
                },
                _ => {
                    anyhow::Error::msg(e)
                }
            }
        })
        ?;
        
        let vvals = Self::from_cfg_str(&cfg)?;

        Ok(vvals)

    }

}
