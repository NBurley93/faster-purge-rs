mod config;
mod faster;
mod logging;
mod scaffold;

fn check_flag(flag: bool) -> String {
    if flag {
        "🟢".to_string()
    } else {
        "⚫".to_string()
    }
}

fn main() {
    let console = logging::ConsoleHandler::new();
    let ver = clap::crate_version!();
    console.log_message(format!("FASTER Purge v{ver}").as_str());
    let (f_version, f_path) = faster::resolve_faster_installation();
    if f_version.is_empty() || f_path.is_empty() {
        console.log_error("Failed to resolve FASTER installation.");
    } else {
        console.log_info(format!("Found FASTER installation V{f_version}").as_str());
        let user_config = match config::read_userconfig_file(f_path) {
            Ok(cfg) => cfg,
            Err(e) => {
                console.log_error(&format!("Failed to read user configuration: {e}"));
                return;
            }
        };
        console.log_info(&format!(
            "{} total mods installed",
            user_config.installed_mods.len()
        ));
        console.log_info(&format!("Found {} profiles.", user_config.profiles.len()));
        console.log_info("Profile Summary:");
        for profile in &user_config.profiles {
            console.log_message(&format!("\nProfile: {}", profile.name));
            let col_width = profile.mods.iter().map(|m| m.name.len()).max().unwrap_or(0);
            for profile_mod in &profile.mods {
                console.log_message(&format!(
                    "  {0:<width$}  [C:{1} S:{2} H:{3} O:{4}]",
                    profile_mod.name,
                    check_flag(profile_mod.markers.client_only),
                    check_flag(profile_mod.markers.server_only),
                    check_flag(profile_mod.markers.headless),
                    check_flag(profile_mod.markers.optional),
                    width = col_width,
                ));
            }
        }
        console.log_info("Identifying Orphaned Mods...");
        let orphans = config::find_orphaned_mods(&user_config);
        if orphans.is_empty() {
            console.log_info("No orphaned mods found.");
        } else {
            console.log_info(&format!("Found {} orphaned mods.", orphans.len()));
            for orphan in orphans {
                console.log_message(format!("  {}", orphan.name).as_str());
            }
        }
        console.log_info("Done!");
    }
}
