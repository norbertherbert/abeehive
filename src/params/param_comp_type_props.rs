use super::ParamType;

pub struct ParamFieldInput {
    pub id: u8,
    pub label: &'static str,
    pub description: &'static str,
    pub default_value: ParamType,
    pub valid_range: (ParamType, ParamType),
}

pub struct ParamFieldHexInput {
    pub id: u8,
    pub label: &'static str,
    pub description: &'static str,
    pub default_value: ParamType,
    pub valid_range: (ParamType, ParamType),
}

pub struct ParamFieldOptionalInput {
    pub id: u8,
    pub label: &'static str,
    pub description: &'static str,
    pub default_value: ParamType,
    pub disabled_value: ParamType,
    pub valid_range: (ParamType, ParamType),
}

#[derive(PartialEq)]
pub struct ParamOptionVariant {
    pub value: ParamType,
    pub text: &'static str,
}
pub struct ParamFieldSelect {
    pub id: u8,
    pub label: &'static str,
    pub description: &'static str,
    pub default_value: ParamType,
    pub options: &'static [ParamOptionVariant],
}

#[derive(PartialEq)]
pub struct ParamBitmapBit {
    pub bit_number: u8,
    pub description: &'static str,
}
pub struct ParamFieldBitmap {
    pub id: u8,
    pub label: &'static str,
    pub description: &'static str,
    pub default_value: ParamType,
    pub bits: &'static [ParamBitmapBit],
}
