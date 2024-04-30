// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{CustomMenuItem, Menu, MenuItem, Submenu};
use tauri::api::dialog;
use tauri::AppHandle;
use tauri::Manager;

use serde::{Deserialize, Serialize};
use std::borrow::Borrow;
use std::fs::File;
use std::io::{Read, Write};
use std::sync::OnceLock;

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
fn open(app: AppHandle) -> () {
    dialog::FileDialogBuilder::default()
        .add_filter("Text", &["txt"])
        .pick_file(move |path_buf| match path_buf {
            Some(file_path) => {
                println!("{:?}", file_path);

                let mut data = Vec::new();
                let mut f = File::open(file_path).expect("Unable to open file");
                f.read_to_end(&mut data).expect("Unable to read data");
                println!("{}", data.len());

                let data = String::from_utf8_lossy(&data[..]);
                let data: &str = data.borrow();

                // println!("{}", data);
                // app.emit_all("FileOpen", data.to_string() ).unwrap();

                let param_values: ParamValues = toml::from_str(data).unwrap();
                dbg!(param_values);
                app.emit_all("FileOpen", param_values ).unwrap();

            }
            _ => {}
        })
}


#[tauri::command(rename_all = "snake_case")]
fn save_as(param_values: ParamValues) -> () {
    let file_content = toml::to_string(&param_values).unwrap();
    dialog::FileDialogBuilder::default()
        .add_filter("Text", &["txt"])
        .save_file(move |path_buf| match path_buf {
            Some(file_path) => {
                println!("{:?}", file_path);
                let mut f = File::create(file_path).expect("Unable to create file");
                f.write_all(file_content.as_bytes()).expect("Unable to write data");
            }
            _ => {}
        });
}


fn main() {

    let open_file = CustomMenuItem::new("open_file".to_string(), "Open file");
    let save_to_file = CustomMenuItem::new("save_to_file".to_string(), "Save to file");
    let open_device = CustomMenuItem::new("open_device".to_string(), "Open device");
    let save_to_device = CustomMenuItem::new("save_to_device".to_string(), "Save to device");
    let file_menu = Submenu::new(
        "File", 
        Menu::new()
            .add_item(open_file)
            .add_item(save_to_file)
            .add_native_item(MenuItem::Separator)
            .add_item(open_device)
            .add_item(save_to_device)
    );
    let menu = Menu::new()
        .add_submenu(file_menu);
        // .add_native_item(MenuItem::Quit);

    tauri::Builder::default()
        .setup(|app| {
            APP_HANDLE.set(app.handle().clone()).unwrap();
            Ok(())
        })
        .menu(menu)
        .on_menu_event(|event| match event.menu_item_id() {
            "open_file" => {
                dialog::FileDialogBuilder::default()
                .add_filter("Text", &["txt"])
                .pick_file(move |path_buf| match path_buf {
                    Some(file_path) => {
                        let mut data = Vec::new();
                        let mut f = File::open(file_path).expect("Unable to open file");
                        f.read_to_end(&mut data).expect("Unable to read data");
        
                        let data = String::from_utf8_lossy(&data[..]);
                        let data: &str = data.borrow();

                        let app = APP_HANDLE.get().unwrap();
                        app.emit_all("MenuClick", ("FileOpen".to_string(), data.to_string()) ).unwrap();
                    }
                    _ => {}
                })
            },
            "save_to_file" => {
                let app = APP_HANDLE.get().unwrap();
                app.emit_all("MenuClick", ("FileSave".to_string(), "".to_string()) ).unwrap();
            },
            _ => { }
        })
        .invoke_handler(tauri::generate_handler![greet, save_as, open])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

}
