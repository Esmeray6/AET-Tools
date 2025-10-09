#![allow(
    non_camel_case_types,
    clippy::upper_case_acronyms,
    clippy::upper_case_acronyms
)]
use strum_macros::{Display, EnumString};

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, EnumString, Display)]
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

    #[strum(serialize = "COY", serialize = "CL", serialize = "Company Leader")]
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
    PSgt,

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

    #[strum(
        serialize = "DEMO",
        serialize = "EOD",
        serialize = "Demolitions",
        serialize = "Demolitions Expert",
        serialize = "Demolitions Specialist",
        serialize = "Explosive_Spec",
        serialize = "Explosive Spec",
        serialize = "Explosives Spec",
        serialize = "Explosives Specialist"
    )]
    #[strum(ascii_case_insensitive)]
    Explosive_Spec,

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

    #[strum(serialize = "Breacher", serialize = "CQB")]
    #[strum(ascii_case_insensitive)]
    Breacher,

    #[strum(serialize = "Pointman", serialize = "Point")]
    #[strum(ascii_case_insensitive)]
    Pointman,

    #[strum(
        serialize = "Rifleman",
        serialize = "Rifle",
        serialize = "RFL",
        serialize = "Scout"
    )]
    #[strum(ascii_case_insensitive)]
    Rifleman,

    #[strum(serialize = "Sniper")]
    #[strum(ascii_case_insensitive)]
    Sniper,

    #[strum(serialize = "Spotter")]
    #[strum(ascii_case_insensitive)]
    Spotter,

    #[strum(
        serialize = "MG_Team",
        serialize = "MGTeam",
        serialize = "MG Team",
        serialize = "AT_Team",
        serialize = "ATTeam",
        serialize = "AT Team",
        serialize = "Heavy_Weapons",
        serialize = "Heavy Weapons",
        serialize = "HeavyWeapons",
        serialize = "HW"
    )]
    #[strum(ascii_case_insensitive)]
    Heavy_Weapons,

    #[strum(serialize = "ARTY", serialize = "Artillery")]
    #[strum(ascii_case_insensitive)]
    ARTY,

    #[strum(serialize = "MORT", serialize = "Mortar")]
    #[strum(ascii_case_insensitive)]
    MORT,

    #[strum(serialize = "LOGI", serialize = "Logistics")]
    #[strum(ascii_case_insensitive)]
    LOGI,

    #[strum(serialize = "MBT", serialize = "Main Battle Tank", serialize = "Tank")]
    #[strum(ascii_case_insensitive)]
    Tank,

    #[strum(serialize = "IFV", serialize = "Infantry Fighting Vehicle")]
    #[strum(ascii_case_insensitive)]
    IFV,

    #[strum(
        serialize = "APC",
        serialize = "Armored Personnel Carrier",
        serialize = "Armoured Personnel Carrier"
    )]
    #[strum(ascii_case_insensitive)]
    APC,

    #[strum(
        serialize = "MRAP",
        serialize = "Mine-Resistant Ambush Protected",
        serialize = "Mine Resistant Ambush Protected",
        serialize = "Car"
    )]
    #[strum(ascii_case_insensitive)]
    Car,

    #[strum(
        serialize = "CAS",
        serialize = "Close Air Support",
        serialize = "Air Support",
        serialize = "CAP",
        serialize = "Combat Air Patrol",
        serialize = "Air Patrol",
        serialize = "Fixed_Wing",
        serialize = "Fixed Wing",
        serialize = "Plane"
    )]
    #[strum(ascii_case_insensitive)]
    Fixed_Wing,

    #[strum(
        serialize = "VTOL",
        serialize = "Vertical Take-Off and Landing",
        serialize = "Vertical Take-Off Landing",
        serialize = "Vertical Take-Off & Landing"
    )]
    #[strum(ascii_case_insensitive)]
    VTOL,

    #[strum(
        serialize = "Rotary",
        serialize = "Rotor",
        serialize = "Rotors",
        serialize = "CAS_Heli",
        serialize = "CASHeli",
        serialize = "CAS Heli",
        serialize = "CAS Helicopter",
        serialize = "Rotary CAS",
        serialize = "Transport",
        serialize = "Transport Heli",
        serialize = "Transport_Heli",
        serialize = "Transport Helo",
        serialize = "Transport_Helo",
        serialize = "Transport Helicopter",
        serialize = "Transport_Helicopter",
        serialize = "Helicopter",
        serialize = "Heli",
        serialize = "Helo"
    )]
    #[strum(ascii_case_insensitive)]
    Rotary,

    #[strum(serialize = "UAV", serialize = "Unmanned Aerial Vehicle")]
    #[strum(ascii_case_insensitive)]
    UAV,

    #[strum(serialize = "Boat")]
    #[strum(ascii_case_insensitive)]
    Boat,

    #[strum(serialize = "Knight")]
    #[strum(ascii_case_insensitive)]
    Knight,

    #[strum(serialize = "Space_Marine", serialize = "Space Marine")]
    #[strum(ascii_case_insensitive)]
    Space_Marine,

    #[strum(serialize = "Spartan")]
    #[strum(ascii_case_insensitive)]
    Spartan,
}
