use std::fmt;
use serde::{Deserialize, Serialize};
use yewdux::Store;

use super::ParamType;
use super::param_id::ParamId;
use super::param_comp_constants::CONF_PARAMS;


#[derive(Serialize, Deserialize, Store, Debug, PartialEq, Clone, Copy)]
pub struct ParamValues {
    pub mode: ParamType,
    pub ul_period: ParamType,
    pub lora_period: ParamType,
    pub periodic_pos_period: ParamType,
    pub geoloc_sensor: ParamType,
    pub geoloc_method: ParamType,
    pub transmit_strat: ParamType,
    pub transmit_strat_custom: ParamType,
    pub config_flags: ParamType,
}
impl fmt::Display for ParamValues {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, 
"
mode = {}
ul_period = {}
lora_period = {}
periodic_pos_period = {}
geoloc_sensor = {}
geoloc_method = {}
transmit_strat = {}
transmit_strat_custom = {}
config_flags = {}
", 
            self.mode,
            self.ul_period,
            self.lora_period,
            self.periodic_pos_period,
            self.geoloc_sensor,
            self.geoloc_method,
            self.transmit_strat,
            self.transmit_strat_custom,
            self.config_flags,
        )
    }
}
impl Default for ParamValues {
    fn default() -> Self {
        ParamValues {
            mode: CONF_PARAMS.mode.default_value,
            ul_period: CONF_PARAMS.ul_period.default_value,
            lora_period: CONF_PARAMS.lora_period.default_value,
            periodic_pos_period: CONF_PARAMS.periodic_pos_period.default_value,
            geoloc_sensor: CONF_PARAMS.geoloc_sensor.default_value,
            geoloc_method: CONF_PARAMS.geoloc_method.default_value,
            transmit_strat: CONF_PARAMS.transmit_strat.default_value,
            transmit_strat_custom: CONF_PARAMS.transmit_strat_custom.default_value,
            config_flags: CONF_PARAMS.config_flags.default_value,
        }
    }
}

pub struct ValueUpdateData {
    pub new_value: ParamType,
    pub param_id: ParamId
}
