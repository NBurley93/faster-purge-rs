// --- XML Scaffolding Structs ---

#[derive(serde::Deserialize)]
pub(crate) struct Configuration {
    #[serde(rename = "userSettings")]
    pub(crate) user_settings: UserSettings,
}

#[derive(serde::Deserialize)]
pub(crate) struct UserSettings {
    #[serde(rename = "FASTER.Properties.Settings")]
    pub(crate) faster_properties_settings: FASTERPropertiesSettings,
}

#[derive(serde::Deserialize)]
pub(crate) struct FASTERPropertiesSettings {
    #[serde(rename = "$value", default)]
    pub(crate) setting: Vec<Setting>,
}

#[derive(serde::Deserialize)]
pub(crate) struct Setting {
    #[serde(rename = "@name")]
    pub(crate) name: String,
    pub(crate) value: Option<SettingValue>,
}

#[derive(serde::Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
pub(crate) struct SettingValue {
    pub(crate) array_of_server_profile: Option<ArrayOfServerProfile>,
    pub(crate) arma_mod_collection: Option<ArmaModCollection>,
}

#[derive(serde::Deserialize, Default)]
pub(crate) struct ArrayOfServerProfile {
    #[serde(rename = "ServerProfile", default)]
    pub(crate) server_profile: Vec<ServerProfile>,
}

#[derive(serde::Deserialize, Default)]
pub(crate) struct ArmaModCollection {
    #[serde(rename = "ArmaMod", default)]
    pub(crate) arma_mod: Vec<RawArmaMod>,
}

// --- Domain Structs ---

// XML order: Id, Name, ...(many fields)..., ProfileMods
// We only need Name and ProfileMods; #[serde(default)] lets quick_xml skip ahead
#[derive(serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
pub(crate) struct ServerProfile {
    #[serde(default)]
    pub(crate) name: String,
    #[serde(default)]
    pub(crate) profile_mods: ProfileMods,
}

#[derive(serde::Deserialize, Default)]
pub(crate) struct ProfileMods {
    #[serde(rename = "ProfileMod", default)]
    pub(crate) profile_mod: Vec<RawProfileMod>,
}

// Fields ordered to match XML element order; unknown fields (LoadPriority, IsLocal) are ignored
#[derive(serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
pub(crate) struct RawProfileMod {
    #[serde(default)]
    pub(crate) server_side_checked: bool,
    #[serde(default)]
    pub(crate) client_side_checked: bool,
    #[serde(default)]
    pub(crate) headless_checked: bool,
    #[serde(default)]
    pub(crate) opt_checked: bool,
    pub(crate) id: String,
    pub(crate) name: String,
}

// XML order: WorkshopId, Name, ...(Author, Path, etc. ignored)
#[derive(serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
pub(crate) struct RawArmaMod {
    pub(crate) workshop_id: String,
    pub(crate) name: String,
}

// --- Output models ---

pub(crate) struct ProfileModMarkers {
    pub client_only: bool,
    pub server_only: bool,
    pub headless: bool,
    pub optional: bool,
}

pub(crate) struct ProfileMod {
    pub id: String,
    pub name: String,
    pub markers: ProfileModMarkers,
}

pub(crate) struct CleanProfile {
    pub(crate) name: String,
    pub(crate) mods: Vec<ProfileMod>,
}

pub(crate) struct ArmaMod {
    pub name: String,
    pub workshop_id: String,
}

pub(crate) struct FASTERUserConfig {
    pub profiles: Vec<CleanProfile>,
    pub installed_mods: Vec<ArmaMod>,
}
