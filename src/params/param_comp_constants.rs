use super::param_bitmaps::{ConfigFlagsBit, TransmitStratCustomBit};
use super::param_comp_type_props::{
    ParamFieldBitmap,
    ParamFieldInput,
    // ParamFieldHexInput,
    ParamFieldOptionalInput,
    ParamFieldSelect,
};
use super::param_options::{
    GeolocMethodOption, GeolocSensorOption, ModeOption, TransmitStratOption,
};

pub struct ConfParams {
    pub mode: ParamFieldSelect,
    pub ul_period: ParamFieldInput,
    pub lora_period: ParamFieldInput,
    pub periodic_pos_period: ParamFieldOptionalInput,
    pub geoloc_sensor: ParamFieldSelect,
    pub geoloc_method: ParamFieldSelect,
    pub transmit_strat: ParamFieldSelect,
    pub transmit_strat_custom: ParamFieldBitmap,
    pub config_flags: ParamFieldBitmap,
}

pub const CONF_PARAMS: ConfParams = ConfParams {
    mode: ParamFieldSelect {
        id: 249,
        label: "Operation Mode",
        description: "Mode Help Text",
        default_value: ModeOption::MOTION_TRACKING.value,
        options: ModeOption::VARIANTS,
    },

    ul_period: ParamFieldInput {
        id: 0,
        label: "Location update period",
        description: "Location update period Help Text",
        default_value: 120,
        valid_range: (1, 10000),
    },

    lora_period: ParamFieldInput {
        id: 1,
        label: "Heartbeat period",
        description: "Heartbeat period Help Text",
        default_value: 300,
        valid_range: (1, 10000),
    },

    periodic_pos_period: ParamFieldOptionalInput {
        id: 3,
        label: "Periodic Position Report Period",
        description: "Periodic Position Report Period Help Text",
        default_value: 3600,
        disabled_value: 0,
        valid_range: (1, 10000),
    },

    geoloc_sensor: ParamFieldSelect {
        id: 5,
        label: "Primary Geoloc Technology",
        description: "Primary Geoloc Technology Help Text",
        default_value: GeolocSensorOption::GPS.value,
        options: GeolocSensorOption::VARIANTS,
    },

    geoloc_method: ParamFieldSelect {
        id: 6,
        label: "Secondary Geoloc Technology",
        description: "Secondary Geoloc Technology Help Text",
        default_value: GeolocSensorOption::WIFI_GPS.value,
        options: GeolocMethodOption::VARIANTS,
    },

    transmit_strat: ParamFieldSelect {
        id: 14,
        label: "Transmit Strategy",
        description: "Transmit Strategy Help Text",
        default_value: TransmitStratOption::DOUBLE_RANDOM.value,
        options: TransmitStratOption::VARIANTS,
    },

    transmit_strat_custom: ParamFieldBitmap {
        id: 30,
        label: "Custom Transmit Strategy Settings",
        description: "Custom Transmit Strategy Help Text",
        default_value: 12289,
        bits: TransmitStratCustomBit::VARIANTS,
    },

    config_flags: ParamFieldBitmap {
        id: 13,
        label: "Configuration Flags",
        description: "Configuration Flags Help Text",
        default_value: 17252411,
        bits: ConfigFlagsBit::VARIANTS,
    },
};
