pub mod my_input;
pub mod my_select;
pub mod my_bitmap;
pub mod my_label;
pub mod my_optional_input;
pub mod myc_transmit_strat_custom;
pub mod myc_motion_sensitivity;
pub mod myc_button_mapping;
pub mod myc_battery_capacity;

pub mod select_usb_port;
pub mod modal;
pub mod navbar;


#[derive(PartialEq)]
pub enum RadixDisp {
    Dec, Hex
}