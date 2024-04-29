//use std::fmt;
use serde::{Deserialize, Serialize};

use super::ParamType;
use super::param_id::ParamId;
use super::param_comp_constants::CONF_PARAMS;


#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum ParamValue {
    Valid(ParamType),
    Invalid(String),
    None
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct ParamValues {
    pub mode: ParamValue,
    pub ul_period: ParamValue,
    pub lora_period: ParamValue,
    pub periodic_pos_period: ParamValue,
    pub geoloc_sensor: ParamValue,
    pub geoloc_method: ParamValue,
    pub transmit_strat: ParamValue,
    pub transmit_strat_custom: ParamValue,
    pub config_flags: ParamValue,
}
impl Default for ParamValues {
    fn default() -> Self {
        ParamValues {
            mode: ParamValue::Valid(CONF_PARAMS.mode.default_value),
            ul_period: ParamValue::Valid(CONF_PARAMS.ul_period.default_value),
            lora_period: ParamValue::Valid(CONF_PARAMS.lora_period.default_value),
            periodic_pos_period: ParamValue::Valid(CONF_PARAMS.periodic_pos_period.default_value),
            geoloc_sensor: ParamValue::Valid(CONF_PARAMS.geoloc_sensor.default_value),
            geoloc_method: ParamValue::Valid(CONF_PARAMS.geoloc_method.default_value),
            transmit_strat: ParamValue::Valid(CONF_PARAMS.transmit_strat.default_value),
            transmit_strat_custom: ParamValue::Valid(CONF_PARAMS.transmit_strat_custom.default_value),
            config_flags: ParamValue::Valid(CONF_PARAMS.config_flags.default_value),
        }
    }
}

pub struct ValueUpdateData {
    pub new_value: ParamValue,
    pub param_id: ParamId
}
