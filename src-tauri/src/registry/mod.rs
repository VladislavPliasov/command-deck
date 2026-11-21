pub mod model;
pub mod service;
pub mod storage;
pub mod validation;
pub mod version_info;

pub use service::RegistryService;

/// Current time as an RFC 3339 / ISO 8601 UTC timestamp
/// (e.g. "2026-11-03T15:04:05Z") without external date dependencies.
pub fn now_rfc3339() -> String {
    let secs = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs() as i64)
        .unwrap_or(0);
    let days = secs.div_euclid(86400);
    let rem = secs.rem_euclid(86400);
    let (y, m, d) = civil_from_days(days);
    format!(
        "{y:04}-{m:02}-{d:02}T{:02}:{:02}:{:02}Z",
        rem / 3600,
        (rem % 3600) / 60,
        rem % 60
    )
}

/// Howard Hinnant's civil_from_days algorithm.
fn civil_from_days(z: i64) -> (i64, u32, u32) {
    let z = z + 719468;
    let era = if z >= 0 { z } else { z - 146096 } / 146097;
    let doe = (z - era * 146097) as u64;
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
    let y = yoe as i64 + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = (doy - (153 * mp + 2) / 5 + 1) as u32;
    let m = if mp < 10 { mp + 3 } else { mp - 9 } as u32;
    (if m <= 2 { y + 1 } else { y }, m, d)
}

#[cfg(test)]
mod tests {
    use super::*;
    use model::ToolProfile;

    #[test]
    fn civil_from_days_known_dates() {
        assert_eq!(civil_from_days(0), (1970, 1, 1));
        assert_eq!(civil_from_days(10957), (2000, 1, 1));
        assert_eq!(civil_from_days(20395), (2025, 11, 3));
    }

    #[test]
    fn now_rfc3339_shape() {
        let s = now_rfc3339();
        assert!(s.ends_with('Z'));
        assert_eq!(s.len(), 20);
        assert_eq!(&s[4..5], "-");
        assert_eq!(&s[7..8], "-");
        assert_eq!(&s[10..11], "T");
    }

    #[test]
    fn example_profile_matches_model_and_validator() {
        // Consistency guard between schemas/tool-profile.schema.json,
        // schemas/examples/tool-profile.example.json and the Rust model:
        // the published example must parse into the model and pass the
        // authoritative Rust validator.
        let raw = include_str!("../../../schemas/examples/tool-profile.example.json");
        let profile: ToolProfile =
            serde_json::from_str(raw).expect("example must deserialize into ToolProfile");
        validation::validate_profile(&profile).expect("example must pass Rust validation");
        assert_eq!(profile.id, "nmap");
        assert_eq!(profile.category.as_str(), "Scanning");
        let commands = profile.commands.expect("example carries commands");
        assert!(commands[0].token.is_none(), "direct-mode entry");
        assert_eq!(commands[0].parameters.len(), 7);
    }

    #[test]
    fn nested_example_profile_matches_model_and_validator() {
        let raw = include_str!("../../../schemas/examples/tool-profile.nested.example.json");
        let profile: ToolProfile =
            serde_json::from_str(raw).expect("nested example must deserialize");
        validation::validate_profile(&profile).expect("nested example must pass Rust validation");
        let commands = profile.commands.expect("nested example carries commands");
        assert_eq!(commands[0].subcommands[0].id, "install");
    }
}
