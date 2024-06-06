use serde::{Deserialize, Serialize};
use std::fmt;
use yewdux::Store;

use serde::{de, ser, Deserializer, Serializer};

use super::param_comp_constants::CONF_PARAMS;
use super::ParamType;

#[derive(Debug, PartialEq, Clone)]
pub enum ParamValue {
    Valid(ParamType),
    Invalid(String),
    None,
}
impl fmt::Display for ParamValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let item_text = match self {
            Self::Valid(v) => v.to_string(),
            Self::Invalid(v) => v.to_string(),
            Self::None => "".to_string(),
        };
        write!(f, "{}", item_text)
    }
}
impl Serialize for ParamValue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            ParamValue::Valid(paramtype) => serializer.serialize_i32(*paramtype),
            _ => Err(ser::Error::custom(
                "ParamValue::Invalid(_) and ParamValue::None variants are not serializable",
            )),
        }
    }
}
impl<'de> Deserialize<'de> for ParamValue {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_i32(ParamValueVisitor)
    }
}

struct ParamValueVisitor;

impl<'de> de::Visitor<'de> for ParamValueVisitor {
    type Value = ParamValue;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("an integer between -2^31 and 2^31")
    }

    fn visit_i32<E>(self, value: i32) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(ParamValue::Valid(value))
    }

    fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        use std::i32;
        if value >= i64::from(i32::MIN) && value <= i64::from(i32::MAX) {
            Ok(ParamValue::Valid(value as i32))
        } else {
            Err(E::custom(format!("i32 out of range: {}", value)))
        }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Store)]
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
            transmit_strat_custom: ParamValue::Valid(
                CONF_PARAMS.transmit_strat_custom.default_value,
            ),
            config_flags: ParamValue::Valid(CONF_PARAMS.config_flags.default_value),
        }
    }
}
impl fmt::Display for ParamValues {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
impl ParamValues {
    pub fn to_cli_cmds(&self) -> Vec<String> {
        let mut cmds: Vec<String> = Vec::new();
        if let ParamValue::Valid(v) = self.mode {
            cmds.push(format!("set {} {}", CONF_PARAMS.mode.id as u8, v))
        }
        if let ParamValue::Valid(v) = self.ul_period {
            cmds.push(format!("set {} {}", CONF_PARAMS.ul_period.id as u8, v))
        }
        if let ParamValue::Valid(v) = self.lora_period {
            cmds.push(format!("set {} {}", CONF_PARAMS.lora_period.id as u8, v))
        }
        if let ParamValue::Valid(v) = self.periodic_pos_period {
            cmds.push(format!(
                "set {} {}",
                CONF_PARAMS.periodic_pos_period.id as u8, v
            ))
        }
        if let ParamValue::Valid(v) = self.geoloc_sensor {
            cmds.push(format!("set {} {}", CONF_PARAMS.geoloc_sensor.id as u8, v))
        }
        if let ParamValue::Valid(v) = self.geoloc_method {
            cmds.push(format!("set {} {}", CONF_PARAMS.geoloc_method.id as u8, v))
        }
        if let ParamValue::Valid(v) = self.transmit_strat {
            cmds.push(format!("set {} {}", CONF_PARAMS.transmit_strat.id as u8, v))
        }
        if let ParamValue::Valid(v) = self.transmit_strat_custom {
            cmds.push(format!(
                "set {} {}",
                CONF_PARAMS.transmit_strat_custom.id as u8, v
            ))
        }
        if let ParamValue::Valid(v) = self.config_flags {
            cmds.push(format!("set {} {}", CONF_PARAMS.config_flags.id as u8, v))
        }
        cmds
    }
}

pub struct ValueUpdateData {
    pub new_param_value: ParamValue,
    pub param_id: u8,
}
