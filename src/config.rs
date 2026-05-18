use crate::scaffold::{ArmaMod, CleanProfile, Configuration, FASTERUserConfig, ProfileMod, ProfileModMarkers};

pub fn read_userconfig_file(faster_path: String) -> Result<FASTERUserConfig, String> {
    let configfile_path = format!("{}\\user.config", faster_path);
    if !std::fs::exists(configfile_path.as_str()).unwrap_or(false) {
        return Err(format!("user.config not found at {configfile_path}"));
    }

    let contents = std::fs::read_to_string(configfile_path)
        .map_err(|e| format!("Failed to read user.config: {e}"))?;
    let parsed: Configuration = quick_xml::de::from_str(&contents)
        .map_err(|e| format!("Failed to parse user.config: {e}"))?;

    let settings = &parsed.user_settings.faster_properties_settings.setting;


    let profiles = settings
        .iter()
        .find(|s| s.name == "Profiles")
        .and_then(|s| s.value.as_ref())
        .and_then(|v| v.array_of_server_profile.as_ref())
        .map(|aosp| {
            aosp.server_profile
                .iter()
                .map(|sp| {
                    let mods = sp
                        .profile_mods
                        .profile_mod
                        .iter()
                        .filter(|m| {
                            m.server_side_checked
                                || m.client_side_checked
                                || m.headless_checked
                                || m.opt_checked
                        })
                        .map(|m| ProfileMod {
                            id: m.id.clone(),
                            name: m.name.clone(),
                            markers: ProfileModMarkers {
                                client_only: m.client_side_checked,
                                server_only: m.server_side_checked,
                                headless: m.headless_checked,
                                optional: m.opt_checked,
                            },
                        })
                        .collect();
                    CleanProfile {
                        name: sp.name.clone(),
                        mods,
                    }
                })
                .collect()
        })
        .unwrap_or_default();

    let installed_mods = settings
        .iter()
        .find(|s| s.name == "armaMods")
        .and_then(|s| s.value.as_ref())
        .and_then(|v| v.arma_mod_collection.as_ref())
        .map(|amc| {
            amc.arma_mod
                .iter()
                .map(|m| ArmaMod {
                    name: m.name.clone(),
                    workshop_id: m.workshop_id.clone(),
                })
                .collect()
        })
        .unwrap_or_default();

    Ok(FASTERUserConfig {
        profiles,
        installed_mods,
    })
}

pub fn find_orphaned_mods(user_config: &FASTERUserConfig) -> Vec<&ArmaMod> {
    user_config
        .installed_mods
        .iter()
        .filter(|installed_mod| {
            !user_config.profiles.iter().any(|profile| {
                profile
                    .mods
                    .iter()
                    .any(|m| m.id == installed_mod.workshop_id)
            })
        })
        .collect()
}
