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
    pub txt: &'static str 
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
    PrmDatHex(PrmDatHex),
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

/// Meta Data for a Parameter Type that can represent a HEXADECIMAL VALUE

pub struct PrmDatHex {
    pub id: u8,
    pub name: &'static str,
    pub label: &'static str,
    pub description: &'static str,
    pub default_val: PrmVal,

    pub range: (PrmVal, PrmVal)
}
impl PrmDat for PrmDatHex {
    fn id(&self) -> u8 { self.id }
    fn name(&self) -> &str { self.name }
    fn vval_from_val(&self, val: PrmVal) -> PrmVVal {
        if self.range.0 <= val && val <= self.range.1 {
            PrmVVal::Valid(val)
        } else {
            PrmVVal::Invalid((val, "invalid value".to_owned()))
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
        
        let mut s = "[".to_string();
        for v in self.distinct_vals {
            s += &format!("{},", v.val)
        }
        s += "]";

        PrmVVal::Invalid((
            val, 
            format!("invalid value: {}, valid values are: {}", val, s)
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
            valid_bitmap |= 1 << b.bit
        }
        if (val & valid_bitmap) == val {
            PrmVVal::Valid(val)
        } else {
            PrmVVal::Invalid((val, "invalid value".to_owned()))
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
