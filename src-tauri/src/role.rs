#![allow(
    non_camel_case_types,
    clippy::upper_case_acronyms,
    clippy::upper_case_acronyms
)]
use strum_macros::EnumString;

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, EnumString)]
pub enum Role {
    #[strum(ascii_case_insensitive)]
    Zeus,

    #[strum(
        serialize = "ZH",
        serialize = "Zeus Helper",
        serialize = "Zeus Help",
        serialize = "ZeusHelp",
        serialize = "ZeusHelper"
    )]
    #[strum(ascii_case_insensitive)]
    ZH,

    #[strum(serialize = "CL", serialize = "Company Leader")]
    #[strum(ascii_case_insensitive)]
    COY,

    #[strum(
        serialize = "PL",
        serialize = "Platoon Leader",
        serialize = "Platoon 1iC"
    )]
    #[strum(ascii_case_insensitive)]
    PL,

    #[strum(
        serialize = "PSgt",
        serialize = "Platoon Sergeant",
        serialize = "Platoon 2iC",
        serialize = "Plt 2iC",
        serialize = "P_Sgt",
        serialize = "P Sgt"
    )]
    #[strum(ascii_case_insensitive)]
    P_Sgt,

    #[strum(serialize = "SL", serialize = "Squad Leader")]
    #[strum(ascii_case_insensitive)]
    SL,

    #[strum(
        serialize = "TL",
        serialize = "Team Leader",
        serialize = "Assistant Squad Leader",
        serialize = "Asst. Squad Leader",
        serialize = "ASL",
        serialize = "Asst SL",
        serialize = "Asst. SL"
    )]
    #[strum(ascii_case_insensitive)]
    TL,

    #[strum(ascii_case_insensitive)]
    RTO,

    #[strum(ascii_case_insensitive)]
    JTAC,

    #[strum(
        serialize = "MED",
        serialize = "Medic",
        serialize = "Corpsman",
        serialize = "Doctor",
        serialize = "Doc"
    )]
    #[strum(ascii_case_insensitive)]
    Medic,

    #[strum(
        serialize = "ENG",
        serialize = "Engi",
        serialize = "Mechanic",
        serialize = "Engineer"
    )]
    #[strum(ascii_case_insensitive)]
    ENG,

    #[strum(ascii_case_insensitive)]
    EOD,

    #[strum(
        serialize = "DEMO",
        serialize = "Demolitions",
        serialize = "Demolitions Expert"
    )]
    #[strum(ascii_case_insensitive)]
    DEMO,

    #[strum(
        serialize = "MG",
        serialize = "Machine Gunner",
        serialize = "Machinegunner"
    )]
    #[strum(ascii_case_insensitive)]
    MG,

    #[strum(
        serialize = "AMG",
        serialize = "Asst. Machine Gunner",
        serialize = "Asst Machine Gunner",
        serialize = "Asst MG",
        serialize = "Assistant MG"
    )]
    #[strum(ascii_case_insensitive)]
    AMG,

    #[strum(
        serialize = "AR",
        serialize = "Auto Rifleman",
        serialize = "Autorifleman"
    )]
    #[strum(ascii_case_insensitive)]
    AR,

    #[strum(
        serialize = "AAR",
        serialize = "Asst. Auto Rifleman",
        serialize = "Asst Auto Rifleman",
        serialize = "Asst. Autorifleman",
        serialize = "Asst Autorifleman",
        serialize = "Assistant Autorifleman"
    )]
    #[strum(ascii_case_insensitive)]
    AAR,

    #[strum(
        serialize = "AT",
        serialize = "Anti-Tank",
        serialize = "Anti Tank",
        serialize = "LAT",
        serialize = "Light Anti-Tank",
        serialize = "Light Anti Tank",
        serialize = "MAT",
        serialize = "Medium Anti-Tank",
        serialize = "Medium Anti Tank",
        serialize = "HAT",
        serialize = "Heavy Anti-Tank",
        serialize = "Heavy Anti Tank"
    )]
    #[strum(ascii_case_insensitive)]
    AT,

    #[strum(
        serialize = "AAT",
        serialize = "Asst Anti-Tank",
        serialize = "Asst AT",
        serialize = "Asst Anti Tank",
        serialize = "Asst. Anti-Tank",
        serialize = "Asst. AT",
        serialize = "Asst. Anti Tank",
        serialize = "Assistant Anti-Tank",
        serialize = "Assistant AntiTank",
        serialize = "Assistant AT"
    )]
    #[strum(ascii_case_insensitive)]
    AAT,

    #[strum(serialize = "AA", serialize = "Anti-Air", serialize = "Anti Air")]
    #[strum(ascii_case_insensitive)]
    AA,

    #[strum(
        serialize = "AAA",
        serialize = "Asst Anti-Air",
        serialize = "Asst AA",
        serialize = "Asst Anti Air",
        serialize = "Asst. Anti-Air",
        serialize = "Asst. AA",
        serialize = "Asst. Anti Air",
        serialize = "Assistant Anti-Air",
        serialize = "Assistant AntiAir",
        serialize = "Assistant AA"
    )]
    #[strum(ascii_case_insensitive)]
    AAA,

    #[strum(serialize = "Pointman", serialize = "Point")]
    #[strum(ascii_case_insensitive)]
    Pointman,

    #[strum(serialize = "DMR", serialize = "Marksman")]
    #[strum(ascii_case_insensitive)]
    DMR,

    #[strum(serialize = "GL", serialize = "Grenadier")]
    #[strum(ascii_case_insensitive)]
    GL,

    #[strum(
        serialize = "AMMO",
        serialize = "Ammo Bearer",
        serialize = "Ammo Bitch"
    )]
    #[strum(ascii_case_insensitive)]
    AMMO,

    #[strum(
        serialize = "Rifleman",
        serialize = "Rifle",
        serialize = "RFL",
        serialize = "Scout"
    )]
    #[strum(ascii_case_insensitive)]
    Rifleman,

    #[strum(serialize = "Sniper Team", serialize = "Sniper_Team")]
    #[strum(ascii_case_insensitive)]
    SniperTeam,

    #[strum(serialize = "MG Team", serialize = "MG_Team")]
    #[strum(ascii_case_insensitive)]
    MGTeam,

    #[strum(serialize = "Artillery", serialize = "ARTY")]
    #[strum(ascii_case_insensitive)]
    Artillery,

    #[strum(serialize = "Logistics", serialize = "LOGI")]
    #[strum(ascii_case_insensitive)]
    Logistics,

    #[strum(serialize = "Main Battle Tank", serialize = "MBT")]
    #[strum(ascii_case_insensitive)]
    MBT,

    #[strum(serialize = "Infantry Fighting Vehicle", serialize = "IFV")]
    #[strum(ascii_case_insensitive)]
    IFV,

    #[strum(
        serialize = "Armored Personnel Carrier",
        serialize = "Armoured Personnel Carrier",
        serialize = "APC"
    )]
    #[strum(ascii_case_insensitive)]
    APC,

    #[strum(
        serialize = "Mine-Resistant Ambush Protected",
        serialize = "Mine Resistant Ambush Protected",
        serialize = "MRAP"
    )]
    #[strum(ascii_case_insensitive)]
    MRAP,

    #[strum(
        serialize = "Close Air Support",
        serialize = "Air Support",
        serialize = "CAS"
    )]
    #[strum(ascii_case_insensitive)]
    CAS,

    #[strum(
        serialize = "Combat Air Patrol",
        serialize = "Air Patrol",
        serialize = "CAP"
    )]
    #[strum(ascii_case_insensitive)]
    CAP,

    #[strum(
        serialize = "Vertical Take-Off and Landing",
        serialize = "Vertical Take-Off Landing",
        serialize = "Vertical Take-Off & Landing",
        serialize = "VTOL"
    )]
    #[strum(ascii_case_insensitive)]
    VTOL,

    #[strum(
        serialize = "CAS Helicopter",
        serialize = "Rotary CAS",
        serialize = "CASHeli",
        serialize = "CAS Heli"
    )]
    #[strum(ascii_case_insensitive)]
    CASHeli,

    #[strum(serialize = "Transport")]
    #[strum(ascii_case_insensitive)]
    Transport,

    #[strum(serialize = "Unmanned Aerial Vehicle", serialize = "UAV")]
    #[strum(ascii_case_insensitive)]
    UAV,
}
