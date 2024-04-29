use std::fmt;


#[repr(u8)]
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ParamId {
    Mode = 0xf9,
    UlPeriod = 0x00,
    LoraPeriod = 0x01,
    PeriodicPosPeriod = 0x03,
    GeolocSensor = 0x05,
    GeolocMethod = 0x06,
    TransmitStrat = 0x0e,
    TransmitStratCustom = 0x1e,
    ConfigFlags = 0x0d,
}
impl fmt::Display for ParamId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        let id_text = match self {
            Self::Mode => { "mode" },
            Self::UlPeriod => { "ul_period" },
            Self::LoraPeriod => { "lora_period" },
            Self::PeriodicPosPeriod => { "periodic_pos_period" },
            Self::GeolocSensor => { "geoloc_sensor" },
            Self::GeolocMethod => { "geoloc_method" },
            Self::TransmitStrat => { "transmit_strat" },
            Self::TransmitStratCustom => { "transmit_strat_custom" },
            Self::ConfigFlags => { "config_flags" },
        };

        write!(f, "{}", id_text)

    }
}
impl ParamId {
    pub fn from_u8(i: u8) -> ParamId {
        match i {
            0xf9 => Self::Mode,
            0x00 => Self::UlPeriod,
            0x01 => Self::LoraPeriod,
            0x03 => Self::PeriodicPosPeriod,
            0x05 => Self::GeolocSensor,
            0x06 => Self::GeolocMethod,
            0x0e => Self::TransmitStrat,
            0x1e => Self::TransmitStratCustom,
            0x0d => Self::ConfigFlags,
            _ => Self::Mode, // TODO!
        }
    }
}


