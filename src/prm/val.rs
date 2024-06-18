use std::collections::BTreeMap;
use anyhow::{anyhow, Result};
use gloo::console::log;

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

        map.insert(MODE.id, PrmVVal::Valid(MODE.default_val));
        map.insert(UL_PERIOD.id, PrmVVal::Valid(UL_PERIOD.default_val));
        map.insert(LORA_PERIOD.id, PrmVVal::Valid(LORA_PERIOD.default_val));
        map.insert(PERIODIC_POS_PERIOD.id, PrmVVal::Valid(PERIODIC_POS_PERIOD.default_val));
        map.insert(GEOLOC_SENSOR.id, PrmVVal::Valid(GEOLOC_SENSOR.default_val));
        map.insert(GEOLOC_METHOD.id, PrmVVal::Valid(GEOLOC_METHOD.default_val));
        map.insert(TRANSMIT_STRAT.id, PrmVVal::Valid(TRANSMIT_STRAT.default_val));
        map.insert(TRANSMIT_STRAT_CUSTOM.id, PrmVVal::Valid(TRANSMIT_STRAT_CUSTOM.default_val));
        map.insert(CONFIG_FLAGS.id, PrmVVal::Valid(CONFIG_FLAGS.default_val));
        map.insert(MOTION_SENSITIVITY.id, PrmVVal::Valid(MOTION_SENSITIVITY.default_val));
        map.insert(BUTTON_MAPPING.id, PrmVVal::Valid(BUTTON_MAPPING.default_val));
        map.insert(BATTERY_CAPACITY.id, PrmVVal::Valid(BATTERY_CAPACITY.default_val));

        Self { vvals_data: map }

    }
}

impl PrmVVals {

    pub fn new() -> Self {
        Self{vvals_data: BTreeMap::new()}
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
    
        let mut vvals = Self::new();
    
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
        let mut vvals = PrmVVals::new();
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
