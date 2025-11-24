// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod role;

use std::{collections::HashMap, str::FromStr};

use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};
use tauri::Manager;
use tauri_plugin_updater::UpdaterExt;

use crate::role::Role;

const REQUIRED_MODS: [&str; 17] = [
    "@ace",
    "@AETAuxiliaries",
    "@AETPlanImporter",
    "@ArmorModifierACE",
    "@CBAA3",
    "@DiwakosPunishunknownweapon",
    "@EnhancedMovement",
    "@EnhancedMovementRework",
    "@FriendlyFirePhantomProtocol",
    "@GruppeAdlerAdminMessages",
    "@MetisMarker",
    "@ProneLauncher",
    "@TaskForceArrowheadRadioBETA",
    "@UserInputMenus",
    "@UVOAETAIO",
    "@VETUnflipping",
    "@ZeusEnhanced",
];

const OPTIONAL_MODS: [&str; 68] = [
    "@3denEnhanced",
    "@A3ThermalImprovement",
    "@AdvancedDeveloperTools",
    "@ACE3ExtensionAnimationsandActions",
    "@ACEInteractionMenuExpansion",
    "@ACEVehicleMedical",
    "@AMZImprovedWSSoundsV78",
    "@AMZIWS3CBFactionsCOMPAT",
    "@AMZIWSCUPCOMPAT",
    "@AMZIWSGlobalMobilizationCOMPAT",
    "@AMZIWSIFA3COMPAT",
    "@AMZIWSRHSAFRFCOMPAT",
    "@AMZIWSRHSAIOCOMPAT",
    "@AMZIWSRHSGREFCOMPAT",
    "@AMZIWSRHSSAFCOMPAT",
    "@AMZIWSRHSUSAFCOMPAT",
    "@AnimateRadio",
    "@AnimateRewrite",
    "@ArmaFXP",
    "@ArmaFXPSmokeEdit",
    "@BetterInventory",
    "@BlastcoreEditedstandaloneversion",
    "@BlastcoreMurrEdition",
    "@CEMovement",
    "@CLVTriggerDebuggerSigned",
    "@CrowsZeusAdditions",
    "@Deformer",
    "@DIRTBloodTextures",
    "@DIRTDynamicTextures",
    "@DUISquadRadar",
    "@DynaSound2",
    "@EnhancedGPS",
    "@EnhancedMapAceVersion",
    "@EnhancedSoundscape",
    "@FawksEnhancedNVGs",
    "@Immerse",
    "@JSRSOPTREunsupported",
    "@JSRSSOUNDMOD",
    "@JSRSSOUNDMODCUPVEHICLESMODSOUNDSUPPORT",
    "@JSRSSOUNDMODCUPWEAPONSMODSOUNDSUPPORT",
    "@JSRSSOUNDMODGlobalMobilizationDLCSounds",
    "@JSRSSOUNDMODIFA3ModSounds",
    "@JSRSSOUNDMODReloadingSounds",
    "@JSRSSOUNDMODRHSAFRFModPackSoundSupport",
    "@JSRSSOUNDMODRHSAiOModPackSoundSupport",
    "@JSRSSOUNDMODRHSGREFModPackSoundSupport",
    "@JSRSSOUNDMODRHSSAFModPackSupport",
    "@JSRSSOUNDMODRHSUSAFModPackSoundSupport",
    "@MRBAirVisibility",
    "@MRBSeaVesselVisibility",
    "@NVGJammer",
    "@RagdollonCommandResigned",
    "@RealisticAutoPilots",
    "@ShackTacUserInterfaceDISCONTINUED",
    "@SOGMeleeCBAKeybind",
    "@SpeshalCore",
    "@Suppress",
    "@TacticalWeaponSwap",
    "@TFARScribbles",
    "@VanillasmokeforBlastcoreEdited",
    "@VileHUD",
    "@VTOLHoverController",
    "@WMOWalkableMovingObjects",
    "@ZECCUPZeusandEdenTemplatesforCUPTerrains",
    "@ZECZeusandEdenTemplatesBuildingCompositions",
    "@ZEIZeusandEdenInteriors",
    "@ZEIZeusandEdenInteriorsContinued",
    "@ZeusAdditions",
];

#[derive(Serialize, Deserialize, Debug, Clone)]
struct ModData {
    mods: String,
    missing_mods: String,
    optional_mods: String,
    dlcs_list: String
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct HEMTTModData {
    mods: String,
    dlcs: String,
    result: String,
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn command_line_convert(modpreset: &str, backticks: bool) -> Result<ModData, String> {
    let mut mod_list = vec![];
    let mut dlcs_list = vec![];

    let markup = Html::parse_document(modpreset);
    let mods_selector =
        Selector::parse("div.mod-list > table > tbody > tr > td[data-type='DisplayName']")
            .expect("No mod list found");
    let dlc_selector =
        Selector::parse("div.dlc-list > table > tbody > tr > td[data-type='DisplayName']")
            .expect("No mod list found");

    for element in markup.select(&dlc_selector) {
        let dlc_name = element.text().next().unwrap();
        dbg!(&dlc_name);
        dlcs_list.push(dlc_name.to_string());
    }

    let mut missing_mods = vec![];
    let mut missing_mods_string = String::new();
    let mut optional_mods = vec![];
    let mut optional_mods_string = String::new();

    for element in markup.select(&mods_selector) {
        let mut mod_name = element.text().next().unwrap().to_string();
        mod_name.retain(|c| c.is_ascii_alphanumeric());
        if !mod_name.starts_with("@") {
            mod_name = format!("@{mod_name}");
        }
        if OPTIONAL_MODS.contains(&&*mod_name) {
            optional_mods.push(mod_name.clone());
            continue;
        }
        mod_list.push(mod_name);
    }

    mod_list.sort_by_key(|a| a.to_lowercase());
    optional_mods.sort_by_key(|a| a.to_lowercase());

    for required_mod in REQUIRED_MODS {
        if !mod_list.contains(&required_mod.to_string()) {
            missing_mods.push(required_mod);
        }
    }
    if !missing_mods.is_empty() {
        missing_mods_string = format!("Required mods missing: {}", missing_mods.join(", "));
    }
    if !optional_mods.is_empty() {
        optional_mods_string = format!("Optional mods found: {}", optional_mods.join(", "));
    }

    let mods = if backticks {
        format!(
            "```\n{}{}\n```",
            mod_list.join(";"),
            if !optional_mods.is_empty() {
                format!(";{}", optional_mods.join(";"))
            } else {
                String::new()
            }
        )
    } else {
        format!(
            "{}{}",
            mod_list.join(";"),
            if !optional_mods.is_empty() {
                format!(";{}", optional_mods.join(";"))
            } else {
                String::new()
            }
        )
    };
    let dlcs_list_string = if !dlcs_list.is_empty() {
        format!("CDLCs found: {}", dlcs_list.join(", "))
    } else {
        String::new()
    };

    Ok(ModData {
        mods,
        missing_mods: missing_mods_string,
        optional_mods: optional_mods_string,
        dlcs_list: dlcs_list_string,
    })
}

#[tauri::command]
async fn orbat_generate(orbat: HashMap<String, u64>) -> Result<String, String> {
    // dbg!(&orbat);
    let mut roles = vec![];

    for (role, amount) in orbat.iter() {
        if !role.is_empty() && *amount > 0 {
            roles.push((format!("{amount}x"), role.to_owned()));
        }
    }

    let (roles_vec, emojis_vec) = convert_roles(roles);

    Ok(dbg!(format!(
        "{}\n\n{}",
        roles_vec.join("\n"),
        emojis_vec.join(" ")
    )))
}

#[tauri::command]
async fn orbat_convert(orbat: String) -> Result<String, String> {
    let mut roles = vec![];

    for line in orbat.lines() {
        let line = line.split_once(" ");
        if let Some(line) = line {
            let (amount, role) = line;
            // let role_enum = Role::from_str(role).expect("Role unable to be converted");
            roles.push((amount.to_string(), role.to_string()));
        }
    }
    let (roles_vec, emojis_vec) = convert_roles(roles);

    Ok(dbg!(format!(
        "{}\n\n{}",
        roles_vec.join("\n"),
        emojis_vec.join(" ")
    )))
}

fn convert_roles(roles: Vec<(String, String)>) -> (Vec<String>, Vec<String>) {
    let mut roles = roles
        .into_iter()
        .filter_map(|role_tuple| match Role::from_str(&role_tuple.1) {
            Ok(role) => Some((
                if role_tuple.0.ends_with("x") {
                    role_tuple.0
                } else {
                    format!("{}x", role_tuple.0)
                },
                role,
            )),
            Err(error) => {
                dbg!(role_tuple, error);
                None
            }
        })
        .collect::<Vec<(String, Role)>>();
    roles.sort_by(|first, second| first.1.cmp(&second.1));

    let zeus_present = dbg!(roles.iter().any(|role| format!("{:?}", role.1)
        .to_string()
        .to_lowercase()
        .contains("zeus")));
    if !zeus_present {
        roles.insert(0, ("2x".to_string(), Role::Zeus));
    }

    let mut roles_vec = vec![];
    let mut emojis_vec = vec![];

    for (role_count, role) in roles {
        roles_vec.push(format!("{role_count} {role:?}"));
        emojis_vec.push(format!(":{role:?}:"));
    }

    (roles_vec, emojis_vec)
}

#[tauri::command]
async fn hemtt_launch_convert(modpreset: String) -> Result<HEMTTModData, String> {
    let mut mod_list = vec![];
    let mut dlc_list = vec![];

    let markup = Html::parse_document(&modpreset);
    let mods_selector =
        Selector::parse("div.mod-list > table > tbody > tr[data-type='ModContainer']")
            .expect("No mod list found");
    let dlc_selector =
        Selector::parse("div.dlc-list > table > tbody > tr[data-type='DlcContainer']")
            .expect("No DLC list found");
    let td_selector = Selector::parse("td").expect("No td found");
    let td_a_selector = Selector::parse("td > a").expect("No td.a found");

    for element in markup.select(&mods_selector) {
        let mut children = element.select(&td_selector);
        // dbg!(&children);
        let mod_name = children.next().unwrap().inner_html();

        let mod_id_td_a = element.select(&td_a_selector).next();
        let mut mod_id = mod_id_td_a
            .expect("mod_id_td_a not found")
            .attr("href")
            .unwrap_or("UNKNOWN MOD ID")
            .to_string();
        mod_id = mod_id
            .strip_prefix("https://steamcommunity.com/sharedfiles/filedetails/?id=")
            .or(mod_id.strip_prefix("http://steamcommunity.com/sharedfiles/filedetails/?id="))
            .unwrap_or("UNKNOWN ID")
            .to_string();
        // mod_name.retain(|c| c.is_alphanumeric());
        mod_list.push((mod_id, mod_name));
    }

    for element in markup.select(&dlc_selector) {
        let mut children = element.select(&td_selector);
        // dbg!(&children);

        let mut dlc_name = children.next().unwrap().inner_html();
        // dlc_name.retain(|c| c.is_alphanumeric());
        dlc_name = format!("\"{dlc_name}\",");
        dlc_list.push(dlc_name);
    }

    mod_list.sort_by_key(|mod_entry| mod_entry.1.to_lowercase());
    dlc_list.sort_by_key(|dlc_entry| dlc_entry.to_lowercase());

    let mods = if !mod_list.is_empty() {
        format!(
            "workshop = [\n{}\n]",
            mod_list
                .iter()
                .map(|(id, name)| format!("\"{id}\", # {name}"))
                .collect::<Vec<String>>()
                .join("\n")
        )
    } else {
        "workshop = []".to_string()
    };
    let dlcs = if !dlc_list.is_empty() {
        format!("dlc = [\n{}\n]", dlc_list.join("\n"))
    } else {
        "dlc = []".to_string()
    };
    dbg!(&dlcs);

    let result = format!("{}\n\n{}", mods, dlcs).trim().to_string();

    Ok(HEMTTModData { mods, dlcs, result })
}

// fn sort_mods(html_preset: String) -> Result<String, String> {
//     let document = Html::parse_document(&html_preset);

//     // Selectors
//     let tr_selector = Selector::parse(r#"html body div.mod-list table tbody tr"#).unwrap();
//     let name_selector = Selector::parse(r#"td[data-type="DisplayName"]"#).unwrap();

//     // Extract and collect (DisplayName, tr_html) tuples
//     let mut mods: Vec<(String, String)> = document
//         .select(&tr_selector)
//         .filter_map(|tr| {
//             let name = tr
//                 .select(&name_selector)
//                 .next()?
//                 .text()
//                 .collect::<String>()
//                 .trim()
//                 .to_string();
//             Some((name, tr.html()))
//         })
//         .collect();

//     // Sort by DisplayName
//     mods.sort_by(|a, b| a.0.to_lowercase().cmp(&b.0.to_lowercase()));

//     let mut final_mods = vec![];

//     for (name, tr_html) in mods {
//         final_mods.push(tr_html);
//     }

//     // Join the sorted HTML strings
//     let sorted_html = final_mods.join("\n");
//     if sorted_html.is_empty() {
//         return Err("No mods found".to_string());
//     }

//     Ok(sorted_html)
// }

fn main() {
    let app = tauri::Builder::default()
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_shell::init())
        .setup(|app| {
            let version = app.package_info().version.to_string();
            let window = app.get_webview_window("main");
            if let Some(window) = window {
                let name = window.title().unwrap_or("AET Tools".to_string());
                // Set the title of the main window
                window
                    .set_title(&format!("{name} v{version}"))
                    .map_err(|error| {
                        eprintln!("Failed to set window title: {error}");
                        error.to_string()
                    })?;
            }

            let handle = app.handle().clone();
            tauri::async_runtime::spawn(async move {
                let response_opt = handle
                    .updater_builder()
                    .build()
                    .unwrap()
                    .check()
                    .await
                    .expect("Error in getting response result");

                if let Some(response) = response_opt {
                    response
                        .download_and_install(
                            |_bytes, _next_bytes| {},
                            || {
                                dbg!("Download finished");
                            },
                        )
                        .await
                        .unwrap();
                } else {
                    dbg!("No updates available");
                }
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            command_line_convert,
            orbat_convert,
            orbat_generate,
            hemtt_launch_convert
        ])
        .build(tauri::generate_context!())
        .expect("error while running tauri application");

    app.run(|_app_handle, _event| {
        // dbg!(&_event);
    });
}
