// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod role;

use std::{collections::HashMap, env::current_dir, fs, str::FromStr};

use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};

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

#[derive(Serialize, Deserialize)]
struct ModData {
    mods: String,
    missing_mods: String,
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

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            command_line_convert,
            orbat_convert
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
