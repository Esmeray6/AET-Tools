// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod role;

use std::{collections::HashMap, env::current_dir, fs, str::FromStr};

use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};

use crate::role::Role;

const REQUIRED_MODS: [&str; 11] = [
    "@ace",
    "@ArmorModifierACE",
    "@CBAA3",
    "@DiwakosPunishunknownweapon",
    "@EnhancedMovement",
    "@EnhancedMovementRework",
    "@MetisMarker",
    "@ProneLauncher",
    "@TaskForceArrowheadRadioBETA",
    "@UVOAETAIO",
    "@ZeusEnhanced",
];

#[derive(Serialize, Deserialize, Debug, Clone)]
struct ModData {
    mods: String,
    missing_mods: String,
}

// #[derive(Serialize, Deserialize, Debug, Clone)]
// struct Entity {
//     #[serde(alias = "dataType")]
//     data_type: String,
//     #[serde(alias = "Attributes")]
//     attributes: Option<EntityAttributes>,
// }

// #[derive(Serialize, Deserialize, Debug, Clone)]
// struct EntityAttributes {
//     rank: Option<String>,
//     description: Option<String>,
//     #[serde(alias = "isPlayable")]
//     is_playable: Option<u8>,
//     #[serde(alias = "isPlayer")]
//     is_player: Option<u8>,
//     #[serde(alias = "Inventory")]
//     inventory: EntityInventory,
// }

// #[derive(Serialize, Deserialize, Debug, Clone)]
// struct EntityInventory {
//     primary_weapon: Option<Weapon>,
//     handgun: Option<Weapon>,
//     secondary_weapon: Option<Weapon>,
//     uniform: Option<Container>,
//     vest: Option<Container>,
//     backpack: Option<Container>,
//     binocular: Option<Item>,
//     compass: Option<String>,
//     gps: Option<String>,
//     map: Option<String>,
//     radio: Option<String>,
//     watch: Option<String>,
//     headgear: Option<String>,
//     goggles: Option<String>,
//     hmd: Option<String>,
// }

// #[derive(Serialize, Deserialize, Debug, Clone)]
// struct Weapon {
//     name: Option<String>,
//     optics: Option<String>,
//     muzzle: Option<String>,
//     flashlight: Option<String>,
//     firemode: Option<String>,
//     #[serde(alias = "primaryMuzzleMag")]
//     primary_muzzle_mag: Option<Item>,
//     #[serde(alias = "secondaryMuzzleMag")]
//     secondary_muzzle_mag: Option<Item>,
// }

// #[derive(Serialize, Deserialize, Debug, Clone)]
// struct Item {
//     name: String,
//     count: Option<u64>,
//     #[serde(alias = "ammoLeft")]
//     ammo_left: Option<u64>,
// }

// #[derive(Serialize, Deserialize, Debug, Clone)]
// struct Container {
//     #[serde(alias = "typeName")]
//     type_name: String,
//     #[serde(alias = "isBackpack")]
//     is_backpack: Option<u64>,
// }

// #[derive(Serialize, Deserialize, Debug, Clone)]
// struct MissionSource {
//     #[serde(alias = "Entities")]
//     entities: Value,
// }

// #[derive(Serialize, Deserialize, Debug, Clone)]
// struct MissionData {
//     sqm: String,
//     players: Vec<Entity>,
// }

// #[derive(Serialize, Deserialize, Debug, Clone)]
// struct MissionPlayers {
//     players: Vec<Entity>,
// }

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn command_line_convert(modpreset: &str, backticks: bool) -> Result<ModData, String> {
    let mut mod_list = vec![];
    let dlc_prefixes: HashMap<String, String> = Default::default();
    let ignored_mods_file = dbg!(current_dir().unwrap().join("ignored_mods.txt"));
    let ignored_mods = fs::read_to_string(ignored_mods_file).unwrap();

    let markup = Html::parse_document(modpreset);
    let mods_selector =
        Selector::parse("div.mod-list > table > tbody > tr > td[data-type='DisplayName']")
            .expect("No mod list found");
    let dlc_selector =
        Selector::parse("div.dlc-list > table > tbody > tr > td[data-type='DisplayName']")
            .expect("No mod list found");

    for element in markup.select(&dlc_selector) {
        let inner_html = element.text().next().unwrap();
        dbg!(&inner_html);
        let dlc_prefix = dlc_prefixes.get(inner_html);
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

    mod_list.sort_by_key(|a| a.to_lowercase());

    let mut missing_mods = vec![];
    let mut missing_mods_str = String::new();
    for required_mod in REQUIRED_MODS {
        if !mod_list.contains(&required_mod.to_string()) {
            missing_mods.push(required_mod);
        }
    }
    if !missing_mods.is_empty() {
        missing_mods_str = format!("Required mods missing: {}", missing_mods.join(", "));
    }

    let mods = if backticks {
        format!("```\n{}\n```", mod_list.join(";"))
    } else {
        mod_list.join(";")
    };

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

    let mut roles = roles
        .into_iter()
        .map(|item| (item.0.to_string(), item.1))
        .collect::<Vec<(String, Role)>>();

    let zeus_present = dbg!(roles.iter().any(|role| format!("{:?}", role.1)
        .to_string()
        .to_lowercase()
        .contains("zeus")));
    if !zeus_present {
        roles.insert(0, ("2x".to_string(), Role::Zeus));
    }

    let emojis = roles
        .iter()
        .map(|role| format!(":{:?}:", role.1))
        .collect::<Vec<String>>();

    Ok(dbg!(format!(
        "{}\n\n{}",
        roles
            .iter()
            .map(|role| { format!("{} {:?}", role.0, role.1) })
            .collect::<Vec<String>>()
            .join("\n")
            .trim(),
        emojis.join(" ")
    )))
}

// #[tauri::command]
// async fn inventory_view(sqm: String, _players: Vec<Value>) -> Result<MissionPlayers, String> {
//     let file_result = File::create("mission.sqm");
//     if let Ok(mut file) = file_result {
//         let write_result = file.write(sqm.as_bytes());
//         if write_result.is_ok() {
//             let _derap_output = Command::new("./MissionDerap.bat")
//                 .args(["mission.sqm"])
//                 .output()
//                 .unwrap();

//             // Successful parsing of the file. The program outputs "{}" if an empty file is received.
//             let json_output = Command::new("./config2json.exe")
//                 .args(["mission.sqm", "output.json"])
//                 .output()
//                 .unwrap();

//             if !json_output.stdout.contains("Parsing") {
//                 return dbg!(Err("Empty file".to_string()));
//             }
//             let file_result = File::open("output.json");
//             if let Ok(mut file) = file_result {
//                 let mut data = String::new();
//                 file.read_to_string(&mut data).unwrap();
//                 let mut players = vec![];

//                 let mut player_inventories = vec![];
//                 let mission_json: Value = serde_json::from_str(&data).unwrap();
//                 let mission_data = serde_json::from_value::<MissionSource>(
//                     mission_json.get("Mission").unwrap().clone(),
//                 )
//                 .unwrap();
//                 let _mission_name = dbg!(&mission_json["sourceName"]
//                     .as_str()
//                     .unwrap()
//                     .replace("_", " "));
//                 let entities = mission_data.entities;
//                 for (_key, value) in entities.as_object().unwrap() {
//                     if value.get("dataType").is_some() {
//                         let data_type = if value.get("dataType").is_some() {
//                             value.get("dataType").unwrap().as_str().unwrap()
//                         } else {
//                             ""
//                         };
//                         if value.is_object() || data_type == "Group" {
//                             if let Some(entity) = value.get("Entities") {
//                                 for (_key, value) in entity.as_object().unwrap() {
//                                     if let Some(attributes) = value.get("Attributes") {
//                                         let is_playable = attributes
//                                             .get("isPlayable")
//                                             .or(attributes.get("isEntity"));
//                                         if is_playable.is_some()
//                                             && is_playable.unwrap().as_u64() == Some(1)
//                                         {
//                                             player_inventories
//                                                 .push(from_value::<Entity>(value.clone()).unwrap())
//                                         }
//                                     }
//                                 }
//                             }
//                         } else if value.is_object() || data_type == "Layer" {
//                             if let Some(entity) = value.get("Entities") {
//                                 for (_key, value) in entity.as_object().unwrap() {
//                                     if value.is_object() || data_type == "Group" {
//                                         if let Some(entity) = value.get("Entities") {
//                                             for (_key, value) in entity.as_object().unwrap() {
//                                                 if let Some(attributes) = value.get("Attributes") {
//                                                     let is_playable = attributes
//                                                         .get("isPlayable")
//                                                         .or(attributes.get("isEntity"));
//                                                     if is_playable.is_some()
//                                                         && is_playable.unwrap().as_u64() == Some(1)
//                                                     {
//                                                         player_inventories.push(
//                                                             from_value::<Entity>(value.clone())
//                                                                 .unwrap(),
//                                                         )
//                                                     }
//                                                 }
//                                             }
//                                         }
//                                     }
//                                 }
//                             }
//                         }
//                     }
//                 }

//                 for player in player_inventories {
//                     // let player_type = player.data_type;
//                     // if player_type != "Object" {
//                     //     continue;
//                     // }

//                     players.push(player);
//                 }

//                 let mission_data_struct = MissionPlayers { players };

//                 // // dbg!(&player_inventories.len());
//                 // // Ensure the parsed value is an object.
//                 Ok(mission_data_struct)
//             } else {
//                 dbg!(Err(file_result.err().unwrap().to_string()))
//             }
//             // return Ok(MissionData { sqm: output.stdout });
//         } else {
//             dbg!(Err(write_result.err().unwrap().to_string()))
//         }
//     } else {
//         Err(file_result.err().unwrap().to_string())
//     }
// }

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            command_line_convert,
            orbat_convert,
            // inventory_view
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
