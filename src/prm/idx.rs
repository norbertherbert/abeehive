use std::collections::BTreeMap;
use std::sync::OnceLock;

use crate::prm::dat::*;
use crate::prm::typ::PrmDat;

static NAME_TO_DATA: OnceLock<BTreeMap<&str, &'static (dyn PrmDat + Sync + Send)>> =
    OnceLock::new();

pub fn name_to_data_map() -> &'static BTreeMap<&'static str, &'static (dyn PrmDat + Sync + Send)> {
    NAME_TO_DATA.get_or_init(|| {
        let mut map: BTreeMap<&str, &'static (dyn PrmDat + Sync + Send)> = BTreeMap::new();

        // map.insert(MODE.name, &MODE);
        // map.insert(UL_PERIOD.name, &UL_PERIOD);
        // map.insert(LORA_PERIOD.name, &LORA_PERIOD);
        // map.insert(PERIODIC_POS_PERIOD.name, &PERIODIC_POS_PERIOD);
        // map.insert(GEOLOC_SENSOR.name, &GEOLOC_SENSOR);
        // map.insert(GEOLOC_METHOD.name, &GEOLOC_METHOD);
        // map.insert(TRANSMIT_STRAT.name, &TRANSMIT_STRAT);
        // map.insert(TRANSMIT_STRAT_CUSTOM.name, &TRANSMIT_STRAT_CUSTOM);
        // map.insert(CONFIG_FLAGS.name, &CONFIG_FLAGS);


        map.insert(UL_PERIOD.name, &UL_PERIOD);
        map.insert(LORA_PERIOD.name, &LORA_PERIOD);
        map.insert(PW_STAT_PERIOD.name, &PW_STAT_PERIOD);
        map.insert(PERIODIC_POS_PERIOD.name, &PERIODIC_POS_PERIOD);
        // map.insert(UNKNOWN.name, &UNKNOWN);
        map.insert(GEOLOC_SENSOR.name, &GEOLOC_SENSOR);
        map.insert(GEOLOC_METHOD.name, &GEOLOC_METHOD);
        // map.insert(ANTENNA.name, &ANTENNA);
        map.insert(MOTION_NB_POS.name, &MOTION_NB_POS);
        map.insert(GPS_TIMEOUT.name, &GPS_TIMEOUT);
        map.insert(AGPS_TIMEOUT.name, &AGPS_TIMEOUT);
        map.insert(GPS_EHPE.name, &GPS_EHPE);
        map.insert(GPS_CONVERGENCE.name, &GPS_CONVERGENCE);
        map.insert(CONFIG_FLAGS.name, &CONFIG_FLAGS);
        map.insert(TRANSMIT_STRAT.name, &TRANSMIT_STRAT);
        map.insert(BLE_BEACON_CNT.name, &BLE_BEACON_CNT);
        map.insert(BLE_BEACON_TIMEOUT.name, &BLE_BEACON_TIMEOUT);
        map.insert(GPS_STANDBY_TIMEOUT.name, &GPS_STANDBY_TIMEOUT);
        map.insert(CONFIRMED_UL_BITMAP.name, &CONFIRMED_UL_BITMAP);
        map.insert(CONFIRMED_UL_RETRY.name, &CONFIRMED_UL_RETRY);
        map.insert(MOTION_SENSITIVITY.name, &MOTION_SENSITIVITY);
        map.insert(SHOCK_DETECTION.name, &SHOCK_DETECTION);
        map.insert(PERIODIC_ACTIVITY_PERIOD.name, &PERIODIC_ACTIVITY_PERIOD);
        map.insert(MOTION_DURATION.name, &MOTION_DURATION);
        map.insert(GEOFENCING_SCAN_PERIOD.name, &GEOFENCING_SCAN_PERIOD);
        map.insert(GEOFENCING_COLLECT_PERIOD.name, &GEOFENCING_COLLECT_PERIOD);
        map.insert(BLE_RSSI_FILTER.name, &BLE_RSSI_FILTER);
        map.insert(TEMPERATURE_HIGH.name, &TEMPERATURE_HIGH);
        map.insert(TEMPERATURE_LOW.name, &TEMPERATURE_LOW);
        map.insert(TEMPERATURE_ACTION.name, &TEMPERATURE_ACTION);
        map.insert(TRANSMIT_STRAT_CUSTOM.name, &TRANSMIT_STRAT_CUSTOM);
        map.insert(NETWORK_TIMEOUT_CHECK.name, &NETWORK_TIMEOUT_CHECK);
        map.insert(NETWORK_TIMEOUT_RESET.name, &NETWORK_TIMEOUT_RESET);
        map.insert(COLLECTION_SCAN_TYPE.name, &COLLECTION_SCAN_TYPE);
        map.insert(COLLECTION_NB_ENTRY.name, &COLLECTION_NB_ENTRY);
        map.insert(COLLECTION_BLE_FILTER_TYPE.name, &COLLECTION_BLE_FILTER_TYPE);
        map.insert(COLLECTION_BLE_FILTER_MAIN_1.name, &COLLECTION_BLE_FILTER_MAIN_1);
        map.insert(COLLECTION_BLE_FILTER_MAIN_2.name, &COLLECTION_BLE_FILTER_MAIN_2);
        map.insert(COLLECTION_BLE_FILTER_SEC_VALUE.name, &COLLECTION_BLE_FILTER_SEC_VALUE);
        map.insert(COLLECTION_BLE_FILTER_SEC_MASK.name, &COLLECTION_BLE_FILTER_SEC_MASK);
        map.insert(BATTERY_CAPACITY.name, &BATTERY_CAPACITY);
        map.insert(REED_SWITCH_CONFIGURATION.name, &REED_SWITCH_CONFIGURATION);
        map.insert(GNSS_CONSTELLATION.name, &GNSS_CONSTELLATION);
        // map.insert(PROX_SCAN_PWR_MIN.name, &PROX_SCAN_PWR_MIN);
        // map.insert(PROX_DISTANCE_COEF.name, &PROX_DISTANCE_COEF);
        // map.insert(PROX_SCAN_FREQUENCY.name, &PROX_SCAN_FREQUENCY);
        // map.insert(PROX_BACKTRACE_MAX_AGE.name, &PROX_BACKTRACE_MAX_AGE);
        // map.insert(PROX_DISTANCE_SLIDING_WINDOW.name, &PROX_DISTANCE_SLIDING_WINDOW);
        // map.insert(PROX_EXPOSURE_50.name, &PROX_EXPOSURE_50);
        // map.insert(PROX_EXPOSURE_100.name, &PROX_EXPOSURE_100);
        // map.insert(PROX_EXPOSURE_150.name, &PROX_EXPOSURE_150);
        // map.insert(PROX_EXPOSURE_200.name, &PROX_EXPOSURE_200);
        // map.insert(PROX_EXPOSURE_250.name, &PROX_EXPOSURE_250);
        // map.insert(PROX_EXPOSURE_300.name, &PROX_EXPOSURE_300);
        // map.insert(PROX_EXPOSURE_400.name, &PROX_EXPOSURE_400);
        // map.insert(PROX_ALARM_DIST_IMMEDIATE.name, &PROX_ALARM_DIST_IMMEDIATE);
        // map.insert(PROX_ALARM_EXPOSURE.name, &PROX_ALARM_EXPOSURE);
        // map.insert(PROX_WARN_DIST_IMMEDIATE.name, &PROX_WARN_DIST_IMMEDIATE);
        // map.insert(PROX_WARN_EXPOSURE.name, &PROX_WARN_EXPOSURE);
        // map.insert(PROX_RECORD_DIST_IMMEDIATE.name, &PROX_RECORD_DIST_IMMEDIATE);
        // map.insert(PROX_RECORD_EXPOSURE.name, &PROX_RECORD_EXPOSURE);
        // map.insert(PROX_ALARM_BUZ_DURATION.name, &PROX_ALARM_BUZ_DURATION);
        // map.insert(PROX_WARN_BUZ_DURATION.name, &PROX_WARN_BUZ_DURATION);
        // map.insert(PROX_CONTACT_POLICY.name, &PROX_CONTACT_POLICY);
        // map.insert(PROX_SCAN_DURATION.name, &PROX_SCAN_DURATION);
        // map.insert(PROX_SCAN_WINDOW.name, &PROX_SCAN_WINDOW);
        // map.insert(PROX_SCAN_INTERVAL.name, &PROX_SCAN_INTERVAL);
        // map.insert(PROX_ALARM_REMANENCE.name, &PROX_ALARM_REMANENCE);
        // map.insert(PROX_WARN_REMANENCE.name, &PROX_WARN_REMANENCE);
        // map.insert(PROX_BCN_REPEAT.name, &PROX_BCN_REPEAT);
        // map.insert(PROX_BCN_TX_POWER.name, &PROX_BCN_TX_POWER);
        // map.insert(PROX_REMINDER_PERIOD.name, &PROX_REMINDER_PERIOD);
        // map.insert(PROX_REMINDER_DISTANCE.name, &PROX_REMINDER_DISTANCE);
        // map.insert(PROX_WARN_DISABLE_DIST.name, &PROX_WARN_DISABLE_DIST);
        // map.insert(PROX_ALARM_DISABLE_DIST.name, &PROX_ALARM_DISABLE_DIST);
        // map.insert(PROX_MAX_SPEED_FILTER.name, &PROX_MAX_SPEED_FILTER);
        // map.insert(PROX_MAX_UPDATE.name, &PROX_MAX_UPDATE);
        map.insert(POSITION_BLE_FILTER_TYPE.name, &POSITION_BLE_FILTER_TYPE);
        map.insert(POSITION_BLE_FILTER_MAIN_1.name, &POSITION_BLE_FILTER_MAIN_1);
        map.insert(POSITION_BLE_FILTER_MAIN_2.name, &POSITION_BLE_FILTER_MAIN_2);
        map.insert(POSITION_BLE_FILTER_SEC_VALUE.name, &POSITION_BLE_FILTER_SEC_VALUE);
        map.insert(POSITION_BLE_FILTER_SEC_MASK.name, &POSITION_BLE_FILTER_SEC_MASK);
        map.insert(POSITION_BLE_REPORT_TYPE.name, &POSITION_BLE_REPORT_TYPE);
        map.insert(BUZZER_VOLUME.name, &BUZZER_VOLUME);
        map.insert(ANGLE_DETECT_MODE.name, &ANGLE_DETECT_MODE);
        map.insert(ANGLE_REF_ACQ.name, &ANGLE_REF_ACQ);
        map.insert(ANGLE_REF_ACC_X.name, &ANGLE_REF_ACC_X);
        map.insert(ANGLE_REF_ACC_Y.name, &ANGLE_REF_ACC_Y);
        map.insert(ANGLE_REF_ACC_Z.name, &ANGLE_REF_ACC_Z);
        map.insert(ANGLE_CRITICAL.name, &ANGLE_CRITICAL);
        map.insert(ANGLE_CRITICAL_HYST.name, &ANGLE_CRITICAL_HYST);
        map.insert(ANGLE_REPORT_MODE.name, &ANGLE_REPORT_MODE);
        map.insert(ANGLE_REPORT_PERIOD.name, &ANGLE_REPORT_PERIOD);
        map.insert(ANGLE_REPORT_REPEAT.name, &ANGLE_REPORT_REPEAT);
        map.insert(ANGLE_RISING_TIME.name, &ANGLE_RISING_TIME);
        map.insert(ANGLE_FALLING_TIME.name, &ANGLE_FALLING_TIME);
        map.insert(ANGLE_LEARNING_TIME.name, &ANGLE_LEARNING_TIME);
        map.insert(ANGLE_ACC_ACCURACY.name, &ANGLE_ACC_ACCURACY);
        map.insert(ANGLE_DEVIATION_DELTA.name, &ANGLE_DEVIATION_DELTA);
        map.insert(ANGLE_DEVIATION_MIN_INTERVAL.name, &ANGLE_DEVIATION_MIN_INTERVAL);
        map.insert(ANGLE_DEVIATION_MAX_INTERVAL.name, &ANGLE_DEVIATION_MAX_INTERVAL);
        map.insert(DEFAULT_PROFILE.name, &DEFAULT_PROFILE);
        map.insert(PASSWORD.name, &PASSWORD);
        map.insert(GPS_T0_TIMEOUT.name, &GPS_T0_TIMEOUT);
        map.insert(GPS_FIX_TIMEOUT.name, &GPS_FIX_TIMEOUT);
        map.insert(GEOFENCING_SCAN_DURATION.name, &GEOFENCING_SCAN_DURATION);
        map.insert(BEACONING_TYPE.name, &BEACONING_TYPE);
        map.insert(BEACONING_TX_POWER.name, &BEACONING_TX_POWER);
        map.insert(BEACONING_STATIC_INTERVAL.name, &BEACONING_STATIC_INTERVAL);
        map.insert(BEACONING_MOTION_INTERVAL.name, &BEACONING_MOTION_INTERVAL);
        map.insert(BEACONING_MOTION_DURATION.name, &BEACONING_MOTION_DURATION);
        map.insert(BLE_CNX_ADV_DURATION.name, &BLE_CNX_ADV_DURATION);
        map.insert(BEACON_ID_0.name, &BEACON_ID_0);
        map.insert(BEACON_ID_1.name, &BEACON_ID_1);
        map.insert(BEACON_ID_2.name, &BEACON_ID_2);
        map.insert(BEACON_ID_3.name, &BEACON_ID_3);
        map.insert(BEACON_ID_4.name, &BEACON_ID_4);
        map.insert(SOS_PERIOD.name, &SOS_PERIOD);
        map.insert(MOTION_DEBOUNCE.name, &MOTION_DEBOUNCE);
        map.insert(BUTTON_MAPPING.name, &BUTTON_MAPPING);
        map.insert(DEFAULT_DATARATE.name, &DEFAULT_DATARATE);
        map.insert(GPS_EHPE_MOTION.name, &GPS_EHPE_MOTION);
        map.insert(GPS_CONVERGENCE_MOTION.name, &GPS_CONVERGENCE_MOTION);
        map.insert(GPS_T0_TIMEOUT_MOTION.name, &GPS_T0_TIMEOUT_MOTION);
        // map.insert(BLE_CLI_ACTIVE.name, &BLE_CLI_ACTIVE);
        map.insert(PROFILE.name, &PROFILE);
        // map.insert(CONSUMPTION.name, &CONSUMPTION);
        // map.insert(BLE_BOND_INFO.name, &BLE_BOND_INFO);
        map.insert(MODE.name, &MODE);
        // map.insert(ACC_X_AXIS.name, &ACC_X_AXIS);
        // map.insert(ACC_Y_AXIS.name, &ACC_Y_AXIS);
        // map.insert(ACC_Z_AXIS.name, &ACC_Z_AXIS);
        // map.insert(BLE_VERSION.name, &BLE_VERSION);
        // map.insert(FIRMWARE_VERSION.name, &FIRMWARE_VERSION);

        map
    })
}

static ID_TO_DATA: OnceLock<BTreeMap<u8, &'static (dyn PrmDat + Sync + Send)>> = OnceLock::new();

pub fn id_to_data_map() -> &'static BTreeMap<u8, &'static (dyn PrmDat + Sync + Send)> {
    ID_TO_DATA.get_or_init(|| {
        let mut map: BTreeMap<u8, &'static (dyn PrmDat + Sync + Send)> = BTreeMap::new();

        // map.insert(MODE.id, &MODE);
        // map.insert(UL_PERIOD.id, &UL_PERIOD);
        // map.insert(LORA_PERIOD.id, &LORA_PERIOD);
        // map.insert(PERIODIC_POS_PERIOD.id, &PERIODIC_POS_PERIOD);
        // map.insert(GEOLOC_SENSOR.id, &GEOLOC_SENSOR);
        // map.insert(GEOLOC_METHOD.id, &GEOLOC_METHOD);
        // map.insert(TRANSMIT_STRAT.id, &TRANSMIT_STRAT);
        // map.insert(TRANSMIT_STRAT_CUSTOM.id, &TRANSMIT_STRAT_CUSTOM);
        // map.insert(CONFIG_FLAGS.id, &CONFIG_FLAGS);

        map.insert(UL_PERIOD.id, &UL_PERIOD);
        map.insert(LORA_PERIOD.id, &LORA_PERIOD);
        map.insert(PW_STAT_PERIOD.id, &PW_STAT_PERIOD);
        map.insert(PERIODIC_POS_PERIOD.id, &PERIODIC_POS_PERIOD);
        // map.insert(UNKNOWN.id, &UNKNOWN);
        map.insert(GEOLOC_SENSOR.id, &GEOLOC_SENSOR);
        map.insert(GEOLOC_METHOD.id, &GEOLOC_METHOD);
        // map.insert(ANTENNA.id, &ANTENNA);
        map.insert(MOTION_NB_POS.id, &MOTION_NB_POS);
        map.insert(GPS_TIMEOUT.id, &GPS_TIMEOUT);
        map.insert(AGPS_TIMEOUT.id, &AGPS_TIMEOUT);
        map.insert(GPS_EHPE.id, &GPS_EHPE);
        map.insert(GPS_CONVERGENCE.id, &GPS_CONVERGENCE);
        map.insert(CONFIG_FLAGS.id, &CONFIG_FLAGS);
        map.insert(TRANSMIT_STRAT.id, &TRANSMIT_STRAT);
        map.insert(BLE_BEACON_CNT.id, &BLE_BEACON_CNT);
        map.insert(BLE_BEACON_TIMEOUT.id, &BLE_BEACON_TIMEOUT);
        map.insert(GPS_STANDBY_TIMEOUT.id, &GPS_STANDBY_TIMEOUT);
        map.insert(CONFIRMED_UL_BITMAP.id, &CONFIRMED_UL_BITMAP);
        map.insert(CONFIRMED_UL_RETRY.id, &CONFIRMED_UL_RETRY);
        map.insert(MOTION_SENSITIVITY.id, &MOTION_SENSITIVITY);
        map.insert(SHOCK_DETECTION.id, &SHOCK_DETECTION);
        map.insert(PERIODIC_ACTIVITY_PERIOD.id, &PERIODIC_ACTIVITY_PERIOD);
        map.insert(MOTION_DURATION.id, &MOTION_DURATION);
        map.insert(GEOFENCING_SCAN_PERIOD.id, &GEOFENCING_SCAN_PERIOD);
        map.insert(GEOFENCING_COLLECT_PERIOD.id, &GEOFENCING_COLLECT_PERIOD);
        map.insert(BLE_RSSI_FILTER.id, &BLE_RSSI_FILTER);
        map.insert(TEMPERATURE_HIGH.id, &TEMPERATURE_HIGH);
        map.insert(TEMPERATURE_LOW.id, &TEMPERATURE_LOW);
        map.insert(TEMPERATURE_ACTION.id, &TEMPERATURE_ACTION);
        map.insert(TRANSMIT_STRAT_CUSTOM.id, &TRANSMIT_STRAT_CUSTOM);
        map.insert(NETWORK_TIMEOUT_CHECK.id, &NETWORK_TIMEOUT_CHECK);
        map.insert(NETWORK_TIMEOUT_RESET.id, &NETWORK_TIMEOUT_RESET);
        map.insert(COLLECTION_SCAN_TYPE.id, &COLLECTION_SCAN_TYPE);
        map.insert(COLLECTION_NB_ENTRY.id, &COLLECTION_NB_ENTRY);
        map.insert(COLLECTION_BLE_FILTER_TYPE.id, &COLLECTION_BLE_FILTER_TYPE);
        map.insert(COLLECTION_BLE_FILTER_MAIN_1.id, &COLLECTION_BLE_FILTER_MAIN_1);
        map.insert(COLLECTION_BLE_FILTER_MAIN_2.id, &COLLECTION_BLE_FILTER_MAIN_2);
        map.insert(COLLECTION_BLE_FILTER_SEC_VALUE.id, &COLLECTION_BLE_FILTER_SEC_VALUE);
        map.insert(COLLECTION_BLE_FILTER_SEC_MASK.id, &COLLECTION_BLE_FILTER_SEC_MASK);
        map.insert(BATTERY_CAPACITY.id, &BATTERY_CAPACITY);
        map.insert(REED_SWITCH_CONFIGURATION.id, &REED_SWITCH_CONFIGURATION);
        map.insert(GNSS_CONSTELLATION.id, &GNSS_CONSTELLATION);
        // map.insert(PROX_SCAN_PWR_MIN.id, &PROX_SCAN_PWR_MIN);
        // map.insert(PROX_DISTANCE_COEF.id, &PROX_DISTANCE_COEF);
        // map.insert(PROX_SCAN_FREQUENCY.id, &PROX_SCAN_FREQUENCY);
        // map.insert(PROX_BACKTRACE_MAX_AGE.id, &PROX_BACKTRACE_MAX_AGE);
        // map.insert(PROX_DISTANCE_SLIDING_WINDOW.id, &PROX_DISTANCE_SLIDING_WINDOW);
        // map.insert(PROX_EXPOSURE_50.id, &PROX_EXPOSURE_50);
        // map.insert(PROX_EXPOSURE_100.id, &PROX_EXPOSURE_100);
        // map.insert(PROX_EXPOSURE_150.id, &PROX_EXPOSURE_150);
        // map.insert(PROX_EXPOSURE_200.id, &PROX_EXPOSURE_200);
        // map.insert(PROX_EXPOSURE_250.id, &PROX_EXPOSURE_250);
        // map.insert(PROX_EXPOSURE_300.id, &PROX_EXPOSURE_300);
        // map.insert(PROX_EXPOSURE_400.id, &PROX_EXPOSURE_400);
        // map.insert(PROX_ALARM_DIST_IMMEDIATE.id, &PROX_ALARM_DIST_IMMEDIATE);
        // map.insert(PROX_ALARM_EXPOSURE.id, &PROX_ALARM_EXPOSURE);
        // map.insert(PROX_WARN_DIST_IMMEDIATE.id, &PROX_WARN_DIST_IMMEDIATE);
        // map.insert(PROX_WARN_EXPOSURE.id, &PROX_WARN_EXPOSURE);
        // map.insert(PROX_RECORD_DIST_IMMEDIATE.id, &PROX_RECORD_DIST_IMMEDIATE);
        // map.insert(PROX_RECORD_EXPOSURE.id, &PROX_RECORD_EXPOSURE);
        // map.insert(PROX_ALARM_BUZ_DURATION.id, &PROX_ALARM_BUZ_DURATION);
        // map.insert(PROX_WARN_BUZ_DURATION.id, &PROX_WARN_BUZ_DURATION);
        // map.insert(PROX_CONTACT_POLICY.id, &PROX_CONTACT_POLICY);
        // map.insert(PROX_SCAN_DURATION.id, &PROX_SCAN_DURATION);
        // map.insert(PROX_SCAN_WINDOW.id, &PROX_SCAN_WINDOW);
        // map.insert(PROX_SCAN_INTERVAL.id, &PROX_SCAN_INTERVAL);
        // map.insert(PROX_ALARM_REMANENCE.id, &PROX_ALARM_REMANENCE);
        // map.insert(PROX_WARN_REMANENCE.id, &PROX_WARN_REMANENCE);
        // map.insert(PROX_BCN_REPEAT.id, &PROX_BCN_REPEAT);
        // map.insert(PROX_BCN_TX_POWER.id, &PROX_BCN_TX_POWER);
        // map.insert(PROX_REMINDER_PERIOD.id, &PROX_REMINDER_PERIOD);
        // map.insert(PROX_REMINDER_DISTANCE.id, &PROX_REMINDER_DISTANCE);
        // map.insert(PROX_WARN_DISABLE_DIST.id, &PROX_WARN_DISABLE_DIST);
        // map.insert(PROX_ALARM_DISABLE_DIST.id, &PROX_ALARM_DISABLE_DIST);
        // map.insert(PROX_MAX_SPEED_FILTER.id, &PROX_MAX_SPEED_FILTER);
        // map.insert(PROX_MAX_UPDATE.id, &PROX_MAX_UPDATE);
        map.insert(POSITION_BLE_FILTER_TYPE.id, &POSITION_BLE_FILTER_TYPE);
        map.insert(POSITION_BLE_FILTER_MAIN_1.id, &POSITION_BLE_FILTER_MAIN_1);
        map.insert(POSITION_BLE_FILTER_MAIN_2.id, &POSITION_BLE_FILTER_MAIN_2);
        map.insert(POSITION_BLE_FILTER_SEC_VALUE.id, &POSITION_BLE_FILTER_SEC_VALUE);
        map.insert(POSITION_BLE_FILTER_SEC_MASK.id, &POSITION_BLE_FILTER_SEC_MASK);
        map.insert(POSITION_BLE_REPORT_TYPE.id, &POSITION_BLE_REPORT_TYPE);
        map.insert(BUZZER_VOLUME.id, &BUZZER_VOLUME);
        map.insert(ANGLE_DETECT_MODE.id, &ANGLE_DETECT_MODE);
        map.insert(ANGLE_REF_ACQ.id, &ANGLE_REF_ACQ);
        map.insert(ANGLE_REF_ACC_X.id, &ANGLE_REF_ACC_X);
        map.insert(ANGLE_REF_ACC_Y.id, &ANGLE_REF_ACC_Y);
        map.insert(ANGLE_REF_ACC_Z.id, &ANGLE_REF_ACC_Z);
        map.insert(ANGLE_CRITICAL.id, &ANGLE_CRITICAL);
        map.insert(ANGLE_CRITICAL_HYST.id, &ANGLE_CRITICAL_HYST);
        map.insert(ANGLE_REPORT_MODE.id, &ANGLE_REPORT_MODE);
        map.insert(ANGLE_REPORT_PERIOD.id, &ANGLE_REPORT_PERIOD);
        map.insert(ANGLE_REPORT_REPEAT.id, &ANGLE_REPORT_REPEAT);
        map.insert(ANGLE_RISING_TIME.id, &ANGLE_RISING_TIME);
        map.insert(ANGLE_FALLING_TIME.id, &ANGLE_FALLING_TIME);
        map.insert(ANGLE_LEARNING_TIME.id, &ANGLE_LEARNING_TIME);
        map.insert(ANGLE_ACC_ACCURACY.id, &ANGLE_ACC_ACCURACY);
        map.insert(ANGLE_DEVIATION_DELTA.id, &ANGLE_DEVIATION_DELTA);
        map.insert(ANGLE_DEVIATION_MIN_INTERVAL.id, &ANGLE_DEVIATION_MIN_INTERVAL);
        map.insert(ANGLE_DEVIATION_MAX_INTERVAL.id, &ANGLE_DEVIATION_MAX_INTERVAL);
        map.insert(DEFAULT_PROFILE.id, &DEFAULT_PROFILE);
        map.insert(PASSWORD.id, &PASSWORD);
        map.insert(GPS_T0_TIMEOUT.id, &GPS_T0_TIMEOUT);
        map.insert(GPS_FIX_TIMEOUT.id, &GPS_FIX_TIMEOUT);
        map.insert(GEOFENCING_SCAN_DURATION.id, &GEOFENCING_SCAN_DURATION);
        map.insert(BEACONING_TYPE.id, &BEACONING_TYPE);
        map.insert(BEACONING_TX_POWER.id, &BEACONING_TX_POWER);
        map.insert(BEACONING_STATIC_INTERVAL.id, &BEACONING_STATIC_INTERVAL);
        map.insert(BEACONING_MOTION_INTERVAL.id, &BEACONING_MOTION_INTERVAL);
        map.insert(BEACONING_MOTION_DURATION.id, &BEACONING_MOTION_DURATION);
        map.insert(BLE_CNX_ADV_DURATION.id, &BLE_CNX_ADV_DURATION);
        map.insert(BEACON_ID_0.id, &BEACON_ID_0);
        map.insert(BEACON_ID_1.id, &BEACON_ID_1);
        map.insert(BEACON_ID_2.id, &BEACON_ID_2);
        map.insert(BEACON_ID_3.id, &BEACON_ID_3);
        map.insert(BEACON_ID_4.id, &BEACON_ID_4);
        map.insert(SOS_PERIOD.id, &SOS_PERIOD);
        map.insert(MOTION_DEBOUNCE.id, &MOTION_DEBOUNCE);
        map.insert(BUTTON_MAPPING.id, &BUTTON_MAPPING);
        map.insert(DEFAULT_DATARATE.id, &DEFAULT_DATARATE);
        map.insert(GPS_EHPE_MOTION.id, &GPS_EHPE_MOTION);
        map.insert(GPS_CONVERGENCE_MOTION.id, &GPS_CONVERGENCE_MOTION);
        map.insert(GPS_T0_TIMEOUT_MOTION.id, &GPS_T0_TIMEOUT_MOTION);
        // map.insert(BLE_CLI_ACTIVE.id, &BLE_CLI_ACTIVE);
        map.insert(PROFILE.id, &PROFILE);
        // map.insert(CONSUMPTION.id, &CONSUMPTION);
        // map.insert(BLE_BOND_INFO.id, &BLE_BOND_INFO);
        map.insert(MODE.id, &MODE);
        // map.insert(ACC_X_AXIS.id, &ACC_X_AXIS);
        // map.insert(ACC_Y_AXIS.id, &ACC_Y_AXIS);
        // map.insert(ACC_Z_AXIS.id, &ACC_Z_AXIS);
        // map.insert(BLE_VERSION.id, &BLE_VERSION);
        // map.insert(FIRMWARE_VERSION.id, &FIRMWARE_VERSION);
        





        map
    })
}

static NAME_TO_ID: OnceLock<BTreeMap<&str, u8>> = OnceLock::new();

pub fn name_to_id_map() -> &'static BTreeMap<&'static str, u8> {
    NAME_TO_ID.get_or_init(|| BTreeMap::from(PARAM_IDS))
}

static ID_TO_NAME: OnceLock<BTreeMap<u8, &str>> = OnceLock::new();

pub fn id_to_name_map() -> &'static BTreeMap<u8, &'static str> {
    ID_TO_NAME.get_or_init(|| {
        let mut map: BTreeMap<u8, &str> = BTreeMap::new();
        for item in PARAM_IDS {
            map.insert(item.1, item.0);
        }
        map
    })
}

pub const PARAM_IDS: [(&str, u8); 125] = [
    ("ul_period", 0),
    ("lora_period", 1),
    ("pw_stat_period", 2),
    ("periodic_pos_period", 3),
    ("unknown", 4),
    ("geoloc_sensor", 5),
    ("geoloc_method", 6),
    ("antenna", 7),
    ("motion_nb_pos", 8),
    ("gps_timeout", 9),
    ("agps_timeout", 10),
    ("gps_ehpe", 11),
    ("gps_convergence", 12),
    ("config_flags", 13),
    ("transmit_strat", 14),
    ("ble_beacon_cnt", 15),
    ("ble_beacon_timeout", 16),
    ("gps_standby_timeout", 17),
    ("confirmed_ul_bitmap", 18),
    ("confirmed_ul_retry", 19),
    ("motion_sensitivity", 20),
    ("shock_detection", 21),
    ("periodic_activity_period", 22),
    ("motion_duration", 23),
    ("geofencing_scan_period", 24),
    ("geofencing_collect_period", 25),
    ("ble_rssi_filter", 26),
    ("temperature_high", 27),
    ("temperature_low", 28),
    ("temperature_action", 29),
    ("transmit_strat_custom", 30),
    ("network_timeout_check", 31),
    ("network_timeout_reset", 32),
    ("collection_scan_type", 33),
    ("collection_nb_entry", 34),
    ("collection_ble_filter_type", 35),
    ("collection_ble_filter_main_1", 36),
    ("collection_ble_filter_main_2", 37),
    ("collection_ble_filter_sec_val", 38),
    ("collection_ble_filter_sec_mas", 39),
    ("battery_capacity", 40),
    ("reed_switch_configuration", 41),
    ("gnss_constellation", 42),
    ("prox_scan_pwr_min", 43),
    ("prox_distance_coef", 44),
    ("prox_scan_frequency", 45),
    ("prox_backtrace_max_age", 46),
    ("prox_distance_sliding_window", 47),
    ("prox_exposure_50", 48),
    ("prox_exposure_100", 49),
    ("prox_exposure_150", 50),
    ("prox_exposure_200", 51),
    ("prox_exposure_250", 52),
    ("prox_exposure_300", 53),
    ("prox_exposure_400", 54),
    ("prox_alarm_dist_immediate", 55),
    ("prox_alarm_exposure", 56),
    ("prox_warn_dist_immediate", 57),
    ("prox_warn_exposure", 58),
    ("prox_record_dist_immediate", 59),
    ("prox_record_exposure", 60),
    ("prox_alarm_buz_duration", 61),
    ("prox_warn_buz_duration", 62),
    ("prox_contact_policy", 63),
    ("prox_scan_duration", 64),
    ("prox_scan_window", 65),
    ("prox_scan_interval", 66),
    ("prox_alarm_remanence", 67),
    ("prox_warn_remanence", 68),
    ("prox_bcn_repeat", 69),
    ("prox_bcn_tx_power", 70),
    ("prox_reminder_period", 71),
    ("prox_reminder_distance", 72),
    ("prox_warn_disable_dist", 73),
    ("prox_alarm_disable_dist", 74),
    ("prox_max_speed_filter", 75),
    ("prox_max_update", 76),
    ("position_ble_filter_type", 77),
    ("position_ble_filter_main_1", 78),
    ("position_ble_filter_main_2", 79),
    ("position_ble_filter_sec_value", 80),
    ("position_ble_filter_sec_mask", 81),
    ("position_ble_report_type", 82),
    ("buzzer_volume", 83),
    ("angle_detect_mode", 84),
    ("angle_ref_acq", 85),
    ("angle_ref_acc_x", 86),
    ("angle_ref_acc_y", 87),
    ("angle_ref_acc_z", 88),
    ("angle_critical", 89),
    ("angle_critical_hyst", 90),
    ("angle_report_mode", 91),
    ("angle_report_period", 92),
    ("angle_report_repeat", 93),
    ("angle_rising_time", 94),
    ("angle_falling_time", 95),
    ("angle_learning_time", 96),
    ("angle_acc_accuracy", 97),
    ("angle_deviation_delta", 98),
    ("angle_deviation_min_interval", 99),
    ("angle_deviation_max_interval", 100),
    ("default_profile", 101),
    ("password", 102),
    ("gps_t0_timeout", 103),
    ("gps_fix_timeout", 104),
    ("geofencing_scan_duration", 105),
    ("beaconing_type", 106),
    ("beaconing_tx_power", 107),
    ("beaconing_static_interval", 108),
    ("beaconing_motion_interval", 109),
    ("beaconing_motion_duration", 110),
    ("ble_cnx_adv_duration", 111),
    ("beacon_id_0", 112),
    ("beacon_id_1", 113),
    ("beacon_id_2", 114),
    ("beacon_id_3", 115),
    ("beacon_id_4", 116),
    ("sos_period", 117),
    ("motion_debounce", 118),
    ("button_mapping", 119),
    ("default_datarate", 120),
    ("gps_ehpe_motion", 121),
    ("gps_convergence_motion", 122),
    ("gps_t0_timeout_motion", 123),
    // ("ble_cli_active", 245),
    // ("profile", 246),
    // ("consumption", 247),
    // ("ble_bond_info", 248),
    ("mode", 249),
    // ("acc_x_axis", 250),
    // ("acc_y_axis", 251),
    // ("acc_z_axis", 252),
    // ("ble_version", 253),
    // ("firmware_version", 254),
];
