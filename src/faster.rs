use regex::Regex;
use std::fs::read_dir;

/*
 * Resolves the installation path of FASTER
 */
pub fn resolve_faster_installation() -> (String, String) {
    let local_appdata = std::env::var("LOCALAPPDATA")
        .unwrap_or_else(|_| "C:\\Users\\Default\\AppData\\Local".to_string());
    let version_re = Regex::new(r"^(\d+)\.(\d+)\.(\d+)\.(\d+)$").unwrap();
    let vendor_base_path = format!(
        "{}\\FoxliCorp\\FASTER_StrongName_r3kmcr0zqf35dnhwrlga5cvn2azjfziz",
        local_appdata
    );
    let mut versions: Vec<(String, u32, String)> = Vec::new();

    if let Ok(entries) = read_dir(&vendor_base_path) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir()
                && let Some(name) = entry.file_name().to_str()
                && let Some(caps) = version_re.captures(name)
            {
                let version_sum = caps[1].parse::<u32>().unwrap_or(0) * 1000
                    + caps[2].parse::<u32>().unwrap_or(0) * 100
                    + caps[3].parse::<u32>().unwrap_or(0) * 10
                    + caps[4].parse::<u32>().unwrap_or(0);
                versions.push((
                    name.to_string(),
                    version_sum,
                    path.to_string_lossy().to_string(),
                ));
            }
        }
    }
    versions.sort_by(|a, b| b.1.cmp(&a.1));
    if let Some(best) = versions.first() {
        (best.0.clone(), best.2.clone())
    } else {
        (String::new(), String::new())
    }
}
