// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod role;

use std::{
    collections::HashMap,
    env::current_dir,
    fs::{self, File},
    io::{Read, Write},
    str::FromStr,
};

use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};
use tauri::api::process::Command;

use crate::role::Role;

const REQUIRED_MODS: [&str; 12] = [
    "@ace",
    "@ArmorModifierACE",
    "@CBAA3",
    "@DiwakosPunishunknownweapon",
    "@EnhancedMovement",
    "@EnhancedMovementRework",
    "@MetisMarker",
    "@ProneLauncher",
    "@TaskForceArrowheadRadioBETA",
    "@UnitVoiceOversAETAiO",
    "@ZeusEnhanced",
    "@ZeusEnhancedACE3Compatibility",
];

#[derive(Serialize, Deserialize, Debug)]
struct ModData {
    mods: String,
    missing_mods: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct WeaponData {
    name: Option<String>,
    primary_muzzle_mag: Option<String>,
    secondary_muzzle_mag: Option<String>,
    flashlight: Option<String>,
    muzzle: Option<String>,
    optics: Option<String>,
    under_barrel: Option<String>,
    firemode: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct InventoryData {
    name: Option<String>,
    display_name: Option<String>,
    count: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug)]
struct PlayerData {
    primary_weapon: Option<WeaponData>,
    handgun: Option<WeaponData>,
    secondary_weapon: Option<WeaponData>,
    uniform: Option<InventoryData>,
    vest: Option<InventoryData>,
    backpack: Option<InventoryData>,
    binocular: Option<String>,
    compass: Option<String>,
    gps: Option<String>,
    map: Option<String>,
    radio: Option<String>,
    watch: Option<String>,
    headgear: Option<String>,
    goggles: Option<String>,
    hmd: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct MissionData {
    sqm: String,
    players: Vec<PlayerData>,
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn command_line_convert(modpreset: &str, backticks: bool) -> Result<ModData, String> {
    let mut mod_list = vec![];
    let dlc_prefixes: HashMap<String, String> = Default::default();
    let ignored_mods_file = dbg!(current_dir().unwrap().join("ignored_mods.txt"));
    let ignored_mods = fs::read_to_string(ignored_mods_file).unwrap();
    let mods;

    let markup = Html::parse_document(&modpreset);
    let mods_selector =
        Selector::parse("div.mod-list > table > tbody > tr > td[data-type='DisplayName']")
            .expect("No mod list found");
    let dlc_selector =
        Selector::parse("div.dlc-list > table > tbody > tr > td[data-type='DisplayName']")
            .expect("No mod list found");

    for element in markup.select(&dlc_selector) {
        let inner_html = element.text().next().unwrap();
        dbg!(&inner_html);
        let dlc_prefix = dlc_prefixes.get(&*inner_html);
        if let Some(dlc_name) = dlc_prefix {
            mod_list.push(dlc_name.to_string());
        }
    }

    for element in markup.select(&mods_selector) {
        let mut mod_name = element.text().next().unwrap().to_string();
        mod_name.retain(|c| c.is_alphanumeric());
        if !mod_name.starts_with("@") {
            mod_name = format!("@{}", mod_name);
        }
        if !ignored_mods.contains(&mod_name) {
            mod_list.push(mod_name);
        } else {
            dbg!(format!("Mod ignored: {}", mod_name));
        }
    }

    mod_list.sort_by(|a, b| a.to_lowercase().cmp(&b.to_lowercase()));

    let mut missing_mods = vec![];
    let mut missing_mods_str = String::new();
    for required_mod in REQUIRED_MODS {
        if !mod_list.contains(&required_mod.to_string()) {
            missing_mods.push(required_mod);
        }
    }
    if missing_mods.len() > 0 {
        missing_mods_str = format!("Required mods missing: {}", missing_mods.join(", "));
    }

    if backticks {
        mods = format!("```\n{}\n```", mod_list.join(";"))
    } else {
        mods = mod_list.join(";");
    }

    Ok(ModData {
        mods,
        missing_mods: missing_mods_str,
    })
}

#[tauri::command]
async fn orbat_convert(orbat: String) -> Result<String, String> {
    let mut roles = vec![];

    for line in orbat.lines() {
        let line = line.split_once(" ");
        if let Some(line) = line {
            let (amount, role) = line;
            dbg!(amount, role);
            let role_enum = Role::from_str(role).expect("Role unable to be converted");
            roles.push((amount, role_enum));
        }
    }

    roles.sort_by(|first, second| first.1.cmp(&second.1));

    let roles = roles
        .into_iter()
        .map(|item| format!("{} {:?}", item.0, item.1))
        .collect::<Vec<String>>();

    Ok(dbg!(roles.join("\n").trim().to_string()))
}

#[tauri::command]
async fn inventory_view(sqm: String) -> Result<MissionData, String> {
    let file_result = File::create("mission.sqm");
    if let Ok(mut file) = file_result {
        let write_result = file.write(sqm.as_bytes());
        if write_result.is_ok() {
            // `new_sidecar()` expects just the filename, NOT the whole path like in JavaScript
            let _derap_output = dbg!(Command::new("./MissionDerap.bat")
                .args(["mission.sqm"])
                .output()
                .unwrap());

            // Successful parsing of the file. The program outputs "{}" if an empty file is received.
            let json_output = dbg!(Command::new("./config2json.exe")
                .args(["mission.sqm", "output.json"])
                .output()
                .unwrap());

            if json_output.stdout.contains("Parsing") {
                let file_result = File::open("output.json");
                if let Ok(mut file) = file_result {
                    let mut data = String::new();
                    file.read_to_string(&mut data).unwrap();
                    let players = vec![];

                    let mission_data_struct = MissionData {
                        sqm: data.clone(),
                        players,
                    };

                    let mut player_inventories = vec![];
                    let mission_json: serde_json::Value = serde_json::from_str(&&data).unwrap();
                    let mission_name =
                        dbg!(&mission_json["sourceName"].to_string().replace("_", " "));
                    let mission_data = mission_json.get("Mission");
                    if let Some(mission_data) = mission_data {
                        let entities = mission_data.get("Entities");
                        if let Some(items) = entities {
                            for (key, value) in items.as_object().unwrap() {
                                if value.get("dataType").is_some() {
                                    let data_type = if value.get("dataType").is_some() {
                                        value.get("dataType").unwrap().as_str().unwrap()
                                    } else {
                                        ""
                                    };
                                    if value.is_object() || data_type == "Group" {
                                        if let Some(entity) = value.get("Entities") {
                                            for (_key, value) in entity.as_object().unwrap() {
                                                if let Some(attributes) = value.get("Attributes") {
                                                    let is_playable = attributes
                                                        .get("isPlayable")
                                                        .or(attributes.get("isPlayer"));
                                                    if is_playable.is_some()
                                                        && is_playable.unwrap().as_u64() == Some(1)
                                                    {
                                                        player_inventories.push(value)
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    dbg!(player_inventories.len());
                    // Ensure the parsed value is an object.
                    return Ok(mission_data_struct);
                } else {
                    return dbg!(Err(file_result.err().unwrap().to_string()));
                }
            } else {
                return dbg!(Err("Empty file".to_string()));
            }
            // return Ok(MissionData { sqm: output.stdout });
        } else {
            return dbg!(Err(write_result.err().unwrap().to_string()));
        }
    } else {
        return Err(file_result.err().unwrap().to_string());
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            command_line_convert,
            orbat_convert,
            inventory_view
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
