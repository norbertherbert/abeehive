// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::api::dialog;
use tauri::Manager;
// use tauri::{CustomMenuItem, Menu, MenuItem, Submenu};

use serde::{Deserialize, Serialize};
use std::borrow::Borrow;
use std::env;
use std::ffi::OsString;
// use std::ffi::OsString;
use std::fs::File;
use std::io::{Read, Write};
use std::path::PathBuf;
use std::sync::OnceLock;

mod serial_port;
use serial_port::{get_config_usb, list_usb_ports, save_config_usb};

pub type ParamType = i32;
#[derive(Serialize, Deserialize, Debug, Default, PartialEq, Clone, Copy)]
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

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command(rename_all = "snake_case")]
fn greet(param_values: ParamValues) -> String {
    // format!("Hello, {}! You've been greeted from Rust!", param_values.mode.to_string())
    toml::to_string(&param_values).unwrap()
}

static APP_HANDLE: OnceLock<tauri::AppHandle> = OnceLock::new();

#[tauri::command(rename_all = "snake_case")]
fn open() -> () {
    dialog::FileDialogBuilder::default()
    .add_filter("Text", &["bwc"])
    .pick_file(move |path_buf| match path_buf {
        Some(file_path) => {
            let mut data = Vec::new();
            let mut f = File::open(file_path.clone()).expect("Unable to open file");
            f.read_to_end(&mut data).expect("Unable to read data");

            let data = String::from_utf8_lossy(&data[..]);
            let data: &str = data.borrow();

            // dbg!(data);

            let file_name = match file_path.file_name() {
                Some(file_name) => {
                    file_name.to_string_lossy().into_owned()
                },
                None => {
                    "unknown file name".to_string()
                }
            };

            let app = APP_HANDLE.get().unwrap();
            app.emit_all("FileOpen", (file_name, data.to_string()))
                .unwrap();

        }
        _ => {}
    })
}

#[tauri::command(rename_all = "snake_case")]
fn open_with() -> () {
    let mut args = env::args();
    match args.next() {
        Some(_) => {},
        None => return (),
    };
    if let Some(file_path) = args.next() {

        let file_path = PathBuf::from(file_path);

        let mut data = Vec::new();
        let mut f = File::open(file_path.clone())
            .expect("Unable to open file");
        f.read_to_end(&mut data).expect("Unable to read data");

        let data = String::from_utf8_lossy(&data[..]);
        let data: &str = data.borrow();

        let file_name = match file_path.file_name() {
            Some(file_name) => {
                file_name.to_string_lossy().into_owned()
            },
            None => {
                "unknown file name".to_string()
            }
        };
        
        // println!("{}\n{}", file_name, data);

        let app = APP_HANDLE.get().unwrap();
        app.emit_all("FileOpen", (file_name, data.to_string())).unwrap();

    }
}

#[tauri::command(rename_all = "snake_case")]
fn save_as(cfg_string: String) -> () {

    let required_file_extension = if cfg_string.starts_with("# LoRaWAN Downlink") {
        "txt"
    } else {
        "bwc"
    };

    dialog::FileDialogBuilder::default()
    .add_filter("Text", &[required_file_extension])
    .save_file(move |path_buf| match path_buf {
        Some(mut file_path) => {

            // Making sure that the file is created with the correct extension
            // Only needed for Linux
            let required_file_extension = OsString::from(required_file_extension);
            if let Some(current_file_extesion) = file_path.extension() {
                if required_file_extension != *current_file_extesion {
                    let mut fp = OsString::from(file_path);
                    fp.push(".");
                    fp.push(required_file_extension);
                    file_path = PathBuf::from(fp);
                }
            } else {
                let mut fp = OsString::from(file_path);
                fp.push(".");
                fp.push(required_file_extension);
                file_path = PathBuf::from(fp);
            }

            let mut f = File::create(file_path.clone()).expect("Unable to create file");
            f.write_all(cfg_string.as_bytes())
                .expect("Unable to write data");

            let file_name = match file_path.file_name() {
                Some(file_name) => {
                    file_name.to_string_lossy().into_owned()
                },
                None => {
                    "unknown file name".to_string()
                }
            };

            let app = APP_HANDLE.get().unwrap();
            app.emit_all("SaveAs", file_name)
                .unwrap();
        }
        _ => {}
    });
}

#[tauri::command(rename_all = "snake_case")]
fn save_config_to_usb_cmd(cli_cmds: Vec<(u8, i32)>, port: String) -> () {
    // dbg!(cli_cmds);
    match save_config_usb(cli_cmds, port) {
        Ok(_) => (),
        Err(err) => {
            eprintln!("failed to open the USB port: {}", err);
            () // TODO: Proper Error Haandling
        }
    }
}

#[tauri::command(rename_all = "snake_case")]
fn get_serial_ports() -> Vec<String> {
    let ports = list_usb_ports();
    ports
}

#[tauri::command(rename_all = "snake_case")]
fn get_config_usb_cmd(port: String) -> Vec<(u8, i32)> {
    match get_config_usb(&port) {
        Ok(config) => config,
        Err(err) => {
            eprintln!("failed to open the USB port: {}", err);
            Vec::new() // TODO: Proper Error Haandling
        }
    }
}

fn main() {

    // let args: Vec<String> = env::args().collect();
    // dbg!(args);

    // let open_file = CustomMenuItem::new("open_file".to_string(), "Open file");
    // let save_to_file = CustomMenuItem::new("save_to_file".to_string(), "Save to file");
    // let open_device = CustomMenuItem::new("open_device".to_string(), "Open device");
    // let save_to_device = CustomMenuItem::new("save_to_device".to_string(), "Save to device");
    // let file_menu = Submenu::new(
    //     "File",
    //     Menu::new()
    //         .add_item(open_file)
    //         .add_item(save_to_file)
    //         .add_native_item(MenuItem::Separator)
    //         .add_item(open_device)
    //         .add_item(save_to_device),
    // );
    // let menu = Menu::new().add_submenu(file_menu);
    // // .add_native_item(MenuItem::Quit);

    tauri::Builder::default()
        .setup(|app| {
            APP_HANDLE.set(app.handle().clone()).unwrap();
            Ok(())
        })
        // .menu(menu)
        .on_menu_event(|event| match event.menu_item_id() {
            "open_file" => {
                open();
            }
            "save_to_file" => {
                let app = APP_HANDLE.get().unwrap();
                app.emit_all("MenuClick", ("FileSave".to_string(), "".to_string()))
                    .unwrap();
            }
            _ => {}
        })
        .invoke_handler(tauri::generate_handler![
            greet,
            save_as,
            open,
            open_with,
            get_serial_ports,
            get_config_usb_cmd,
            save_config_to_usb_cmd,
            
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

}
