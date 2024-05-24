use strum_macros::EnumString;

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, EnumString)]
pub enum Role {
    #[strum(ascii_case_insensitive)]
    Zeus,

    #[strum(serialize = "ZH", serialize = "Zeus Helper", serialize = "Zeus Help")]
    #[strum(ascii_case_insensitive)]
    ZeusHelper,

    #[strum(serialize = "CL", serialize = "Company Leader")]
    #[strum(ascii_case_insensitive)]
    Coy,

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
        serialize = "Plt 2iC"
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
    Engineer,

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

    #[strum(serialize = "AT", serialize = "Anti-Tank", serialize = "Anti Tank")]
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
}
