//! The settings of Preferences > Updates (ADR 0068).

use birchpad_config::*;
use toml::Table;

fn table(toml: &str) -> Option<Table> {
    Some(toml::from_str(toml).unwrap())
}

#[test]
fn updates_are_off_until_turned_on() {
    // This build has no update service of its own: a check would fail closed, so none runs
    // until the user turns updates on (ADR 0068).
    assert_eq!(UpdateSettings::default().mode, UpdateMode::Off);
    assert_eq!(Settings::default().updates.mode, UpdateMode::Off);
    // Given, the mode holds.
    let settings: Settings = toml::from_str("[updates]\nmode = \"notify\"\n").unwrap();
    assert_eq!(settings.updates.mode, UpdateMode::Notify);
    let settings: Settings = toml::from_str("[updates]\nmode = \"auto\"\n").unwrap();
    assert_eq!(settings.updates.mode, UpdateMode::Auto);
    check_setting("updates.mode", &toml::Value::String("auto".into())).unwrap();
    // Not a mode: refused, and the default stays.
    for wrong in [
        toml::Value::String("sometimes".into()),
        toml::Value::Integer(0),
    ] {
        assert!(check_setting("updates.mode", &wrong).is_err(), "{wrong}");
    }
    let resolved = resolve(Sources {
        user: table("updates.mode = 'sometimes'"),
        ..Sources::default()
    });
    assert_eq!(resolved.settings.updates.mode, UpdateMode::Off);
}

#[test]
fn the_mode_policy_wins_over_the_setting() {
    let resolved = resolve(Sources {
        machine: table("updates.mode = \"auto\""),
        user: table("updates.mode = \"auto\""),
        policy: table("updates.mode = \"off\""),
        ..Sources::default()
    });
    assert_eq!(resolved.settings.updates.mode, UpdateMode::Off);
    assert!(resolved.is_locked("updates.mode"));
    // Without the policy the user's setting holds, and is not locked.
    let resolved = resolve(Sources {
        user: table("updates.mode = \"notify\""),
        ..Sources::default()
    });
    assert_eq!(resolved.settings.updates.mode, UpdateMode::Notify);
    assert!(!resolved.is_locked("updates.mode"));
}

#[test]
fn the_update_policies_are_listed_with_their_keys() {
    for (name, key, kind) in [
        ("UpdateMode", "updates.mode", PolicyKind::Text),
        ("UpdateChannel", "updates.channel", PolicyKind::Text),
        ("UpdateUrl", "updates.url", PolicyKind::Text),
    ] {
        let policy = POLICIES
            .iter()
            .find(|policy| policy.name == name)
            .unwrap_or_else(|| panic!("{name} is listed, so that templates have it"));
        assert_eq!(policy.key, key, "{name}");
        assert_eq!(policy.kind, kind, "{name}");
    }
    // Names and keys stay unique.
    for (index, policy) in POLICIES.iter().enumerate() {
        assert!(
            POLICIES[index + 1..]
                .iter()
                .all(|other| other.name != policy.name && other.key != policy.key),
            "{}",
            policy.name
        );
    }
}
