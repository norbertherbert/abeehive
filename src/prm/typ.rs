use regex::Regex;

use crate::prm::val::PrmVVal;

pub type PrmVal = i32;


fn interim_vval_from_txt(txt: &str) -> PrmVVal {
    if txt.is_empty() {
        return PrmVVal::InvalidTxt(("".to_owned(), "This field musn't be empty.".to_owned()));
    }
    let regex_for_dec = Regex::new("^[0-9]+$").unwrap();
    let regex_for_hex = Regex::new("^(0x){1}[a-fA-F0-9]+$").unwrap();
    if regex_for_dec.is_match(txt) {
        let Ok(val) = PrmVal::from_str_radix(txt, 10) else {
            return PrmVVal::InvalidTxt((
                txt.to_owned(), 
                format!("Cannot convert the input text to a 32 bit number: \"{}\"", txt),
            ));
        };
        PrmVVal::Valid(val) // Interim value. Still to be validated by the trait's vval_from_val() method!
    } 
    else if regex_for_hex.is_match(txt) {
        let txt = txt.strip_prefix("0x").expect("never fails, secured by regex");
        let Ok(val) = PrmVal::from_str_radix(txt, 16) else {
            return PrmVVal::InvalidTxt((
                txt.to_owned(), 
                format!("Cannot convert the input text to a 32 bit number: \"{}\"", txt),
            ));
        };
        PrmVVal::Valid(val) // Interim value. Still to be validated by the trait's vval_from_val() method!
    }
    else {
        return PrmVVal::InvalidTxt((
            txt.to_owned(),
            format!("Invalid number format: \"{}\", (Should be decimal or hexadecimal.)", txt)
        ));
    }
}

#[derive(PartialEq)]
pub struct BitmapBit { 
    pub bit: u8, 
    pub txt: &'static str,
    pub ena: bool,
}

#[derive(PartialEq)]
pub struct DistinctVal {
    pub val: PrmVal, 
    pub txt: &'static str
}


/// A trait represent the common properties of all Parameter Types

pub trait PrmDat {
    fn id(&self) -> u8;
    fn name(&self) -> &str;
    fn vval_from_val(&self, val: PrmVal) -> PrmVVal;
    fn vval_from_txt(&self, txt: &str) -> PrmVVal;
}


// An Enum thaat can represent any Parameter Types

pub enum PrmDatEnum {
    PrmDatDec(PrmDatDec),
    // PrmDatHex(PrmDatHex),
    PrmDatOptional(PrmDatOptional),
    PrmDatDistinct(PrmDatDistinct),
    PrmDatBitmap(PrmDatBitmap),
}


/// Meta Data for a Parameter Type that can represent a DECIMAL VALUE

pub struct PrmDatDec {
    pub id: u8,
    pub name: &'static str,
    pub label: &'static str,
    pub description: &'static str,
    pub default_val: PrmVal,

    pub range: (PrmVal, PrmVal)
}
impl PrmDat for PrmDatDec {
    fn id(&self) -> u8 { self.id }
    fn name(&self) -> &str { self.name }
    fn vval_from_val(&self, val: PrmVal) -> PrmVVal {
        if self.range.0 <= val && val <= self.range.1 { 
            PrmVVal::Valid(val)
        } else {
            PrmVVal::Invalid((
                val, 
                format!(
                    "invalid value, should be in range [{}..{}]", 
                    self.range.0, self.range.1
                ),
            ))
        }
    }
    fn vval_from_txt(&self, txt: &str) -> PrmVVal {
        match interim_vval_from_txt(txt) {
            PrmVVal::Valid(val) => Self::vval_from_val(self, val),
            PrmVVal::Invalid(v) => PrmVVal::Invalid(v),
            PrmVVal::InvalidTxt(v) => PrmVVal::InvalidTxt(v),
        }
    }
}


/// Meta Data for a Parameter Type that can represent an OPTIONAL VALUE

pub struct PrmDatOptional {
    pub id: u8,
    pub name: &'static str,
    pub label: &'static str,
    pub description: &'static str,
    pub default_val: PrmVal,

    pub disabled_val: PrmVal,
    pub range: (PrmVal, PrmVal)
}
impl PrmDat for PrmDatOptional {
    fn id(&self) -> u8 { self.id }
    fn name(&self) -> &str { self.name }
    fn vval_from_val(&self, val: PrmVal) -> PrmVVal {
        if (self.range.0 <= val && val <= self.range.1) || val == self.disabled_val {
            PrmVVal::Valid(val)
        } else {
            PrmVVal::Invalid((
                val, 
                format!(
                    "invalid value, should be in range [{}..{}]", 
                    self.range.0, self.range.1
                ),
            ))
        }
    }
    fn vval_from_txt(&self, txt: &str) -> PrmVVal {
        match interim_vval_from_txt(txt) {
            PrmVVal::Valid(val) => Self::vval_from_val(self, val),
            PrmVVal::Invalid(v) => PrmVVal::Invalid(v),
            PrmVVal::InvalidTxt(v) => PrmVVal::InvalidTxt(v),
        }
    }
}


/// Meta Data for a Parameter Type that can represent a DISTINCT VALUE

pub struct PrmDatDistinct {
    pub id: u8,
    pub name: &'static str,
    pub label: &'static str,
    pub description: &'static str,
    pub default_val: PrmVal,

    pub distinct_vals: &'static [DistinctVal], 
}
impl PrmDat for PrmDatDistinct {
    fn id(&self) -> u8 { self.id }
    fn name(&self) -> &str { self.name }
    fn vval_from_val(&self, val: PrmVal) -> PrmVVal {
        for v in self.distinct_vals {
            if v.val == val {
                return PrmVVal::Valid(val);
            }
        }
        
        let mut s = String::new();
        for v in self.distinct_vals {
            s += &format!("{},", v.val)
        }

        PrmVVal::Invalid((
            val, 
            format!("invalid value: {}, valid values are: [{}]", val, s)
        ))

    }
    fn vval_from_txt(&self, txt: &str) -> PrmVVal {
        match interim_vval_from_txt(txt) {
            PrmVVal::Valid(val) => Self::vval_from_val(self, val),
            PrmVVal::Invalid(v) => PrmVVal::Invalid(v),
            PrmVVal::InvalidTxt(v) => PrmVVal::InvalidTxt(v),
        }
    }
}


/// Meta Data for a Parameter Type that can represents a BITMAP

pub struct PrmDatBitmap {
    pub id: u8,
    pub name: &'static str,
    pub label: &'static str,
    pub description: &'static str,
    pub default_val: PrmVal,

    pub bits: &'static [BitmapBit],
}
impl PrmDat for PrmDatBitmap {
    fn id(&self) -> u8 { self.id }
    fn name(&self) -> &str { self.name }
    fn vval_from_val(&self, val: PrmVal) -> PrmVVal {
        
        let mut valid_bitmap: PrmVal = 0;
        for b in self.bits {
            if b.ena {
                valid_bitmap |= 1 << b.bit
            }
        }

        if (val & valid_bitmap) == val {
            PrmVVal::Valid(val)
        } else {
            PrmVVal::Invalid((
                val, 
                format!("invalid bit set in bitmap, the valid bitmask is: 0b{:025b}", valid_bitmap)
            ))
        }
    }
    fn vval_from_txt(&self, txt: &str) -> PrmVVal {
        match interim_vval_from_txt(txt) {
            PrmVVal::Valid(val) => Self::vval_from_val(self, val),
            PrmVVal::Invalid(v) => PrmVVal::Invalid(v),
            PrmVVal::InvalidTxt(v) => PrmVVal::InvalidTxt(v),
        }
    }
}



/// Custom types

/// Meta Data for a Parameter Type that can represent a MOTION_SENSITIVITY VALUE

pub struct PrmDatMotionSensitivity {
    pub id: u8,
    pub name: &'static str,
    pub label: &'static str,
    pub description: &'static str,
    pub default_val: PrmVal,
    pub range_sensitivity: (u8, u8),
    pub distinct_vals_odr: &'static [DistinctVal],
    pub distinct_vals_fullscale: &'static [DistinctVal],
}
impl PrmDat for PrmDatMotionSensitivity {
    fn id(&self) -> u8 { self.id }
    fn name(&self) -> &str { self.name }
    fn vval_from_val(&self, val: PrmVal) -> PrmVVal {

        let mut err = String::new();
        let mut is_err = false;
        let octets = val.to_le_bytes();

        if
            octets[0] < self.range_sensitivity.0 ||
            octets[0] > self.range_sensitivity.1
        {
            err += "octet_0: invalid Sensitivity value, ";
            is_err = true;
        } else {
            err += "octet_0: OK, ";
        }

        if 
            !self.distinct_vals_odr.iter()
            .filter(|item| item.val == octets[1] as PrmVal)
            .next()
            .is_some()
        {
            err += "octet_1: invalid ODR value, ";
            is_err = true;
        } else {
            err += "octet_1: OK, ";
        }

        if 
            !self.distinct_vals_fullscale.iter()
            .filter(|item| item.val == octets[2] as PrmVal)
            .next()
            .is_some()
        {
            err += "octet_2: invalid Full Scale value, ";
            is_err = true;
        } else {
            err += "octet_2: OK, ";
        }

        if is_err {
            PrmVVal::Invalid((val, err))
        } else {
            PrmVVal::Valid(val)
        }

    }
    fn vval_from_txt(&self, txt: &str) -> PrmVVal {
        match interim_vval_from_txt(txt) {
            PrmVVal::Valid(val) => Self::vval_from_val(self, val),
            PrmVVal::Invalid(v) => PrmVVal::Invalid(v),
            PrmVVal::InvalidTxt(v) => PrmVVal::InvalidTxt(v),
        }
    }
}



/// Meta Data for a Parameter Type that can represent a BUTTON_MAPPING VALUE

pub struct PrmDatButtonMapping {
    pub id: u8,
    pub name: &'static str,
    pub label: &'static str,
    pub description: &'static str,
    pub default_val: PrmVal,
    pub long_press_duration_range: (u8, u8),
    pub action_distinct_vals: &'static [DistinctVal], 
}
impl PrmDat for PrmDatButtonMapping {
    fn id(&self) -> u8 { self.id }
    fn name(&self) -> &str { self.name }
    fn vval_from_val(&self, val: PrmVal) -> PrmVVal {

        let mut v = val;
        let long_press_action = (v as u8) & 0x0f;
        v = v >> 4;
        let short_press_action = (v as u8) & 0x0f;
        v = v >> 4;
        let short2_press_action = (v as u8) & 0x0f;
        v = v >> 4;
        let short3_press_action = (v as u8) & 0x0f;
        v = v >> 4;
        let long_press_duration = (v as u8) & 0x0f;

        let mut err = String::new();

        if 
            !self.action_distinct_vals.iter()
            .filter(|item| item.val == long_press_action as PrmVal)
            .next()
            .is_some()
        {
            err += "invalid Long Press Action, ";
        }

        if 
            !self.action_distinct_vals.iter()
            .filter(|item| item.val == short_press_action as PrmVal)
            .next()
            .is_some()
        {
            err += "invalid Short Press Action, ";
        }

        if 
            !self.action_distinct_vals.iter()
            .filter(|item| item.val == short2_press_action as PrmVal)
            .next()
            .is_some()
        {
            err += "invalid Double Short Press Action, ";
        }

        if 
            !self.action_distinct_vals.iter()
            .filter(|item| item.val == short3_press_action as PrmVal)
            .next()
            .is_some()
        {
            err += "invalid Tripple Short Press Action, ";
        }

        if
            long_press_duration < self.long_press_duration_range.0 ||
            long_press_duration > self.long_press_duration_range.1
        {
            err += "invalid Long Press Duration value, ";
        }

        if err.is_empty() {
            PrmVVal::Valid(val)
        } else {
            PrmVVal::Invalid((val, err))
        }

    }
    fn vval_from_txt(&self, txt: &str) -> PrmVVal {
        match interim_vval_from_txt(txt) {
            PrmVVal::Valid(val) => Self::vval_from_val(self, val),
            PrmVVal::Invalid(v) => PrmVVal::Invalid(v),
            PrmVVal::InvalidTxt(v) => PrmVVal::InvalidTxt(v),
        }
    }
}


/// Meta Data for a Parameter Type that can represent a BATTERY_CAPACITY VALUE

pub struct PrmDatBatteryCapacity {
    pub id: u8,
    pub name: &'static str,
    pub label: &'static str,
    pub description: &'static str,
    pub default_val: PrmVal,
    pub range: (PrmVal, PrmVal),

    pub distinct_vals: &'static [DistinctVal], 
}
impl PrmDat for PrmDatBatteryCapacity {
    fn id(&self) -> u8 { self.id }
    fn name(&self) -> &str { self.name }
    fn vval_from_val(&self, val: PrmVal) -> PrmVVal {

        if self.range.0 <= val && val <= self.range.1 {
            return PrmVVal::Valid(val);
        }

        for v in self.distinct_vals {
            if v.val == val {
                return PrmVVal::Valid(val);
            }
        }
        
        let mut s = String::new();
        for v in self.distinct_vals {
            s += &format!("{},", v.val)
        }

        PrmVVal::Invalid((
            val, 
            format!(
                "invalid value: {}, valid values are: [{}]U[{}..{}]", 
                val, s, self.range.0, self.range.1
            )
        ))

    }
    fn vval_from_txt(&self, txt: &str) -> PrmVVal {
        match interim_vval_from_txt(txt) {
            PrmVVal::Valid(val) => Self::vval_from_val(self, val),
            PrmVVal::Invalid(v) => PrmVVal::Invalid(v),
            PrmVVal::InvalidTxt(v) => PrmVVal::InvalidTxt(v),
        }
    }
}