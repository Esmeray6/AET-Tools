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

const OPTIONAL_MODS: [&str; 66] = [
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
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn command_line_convert(modpreset: &str, backticks: bool) -> Result<ModData, String> {
    let mut mod_list = vec![];
    let dlc_prefixes: HashMap<String, String> = Default::default();

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

    let mut missing_mods = vec![];
    let mut missing_mods_string = String::new();
    let mut optional_mods = vec![];
    let mut optional_mods_string = String::new();

    for element in markup.select(&mods_selector) {
        let mut mod_name = element.text().next().unwrap().to_string();
        mod_name.retain(|c| c.is_alphanumeric());
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

    Ok(ModData {
        mods,
        missing_mods: missing_mods_string,
        optional_mods: optional_mods_string,
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
            orbat_generate
        ])
        .build(tauri::generate_context!())
        .expect("error while running tauri application");

    app.run(|_app_handle, _event| {
        // dbg!(&_event);
    });
}
