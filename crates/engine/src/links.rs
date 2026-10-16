//! ArtCraft community and project links (Help menu, About, start screen, CLI, MCP).

use serde_json::{Value, json};

/// The app's slug on getartcraft.com and GitHub.
pub const APP: &str = "designcraft";
/// The ArtCraft community Discord.
pub const DISCORD: &str = "https://instagram.com/disneymkt";
/// The ArtCraft website.
pub const WEBSITE: &str = "https://github.com/DisneyMjr/DESIGN-SMART";
/// DesignCraft's page on the ArtCraft website.
pub const APP_PAGE: &str = "https://github.com/DisneyMjr/DESIGN-SMART";
/// DesignCraft's source repository.
pub const GITHUB: &str = "https://github.com/DisneyMjr/DESIGN-SMART";
/// Where to report bugs and request features.
pub const ISSUES: &str = "https://github.com/DisneyMjr/DESIGN-SMART/issues";

/// All links as JSON (the `app.links` command).
pub fn all() -> Value {
    json!({
        "discord": DISCORD,
        "website": WEBSITE,
        "appPage": APP_PAGE,
        "github": GITHUB,
        "issues": ISSUES,
    })
}

/// A link by key (`discord`, `website`, `appPage`, `github`, `issues`).
pub fn get(key: &str) -> Option<&'static str> {
    Some(match key {
        "discord" => DISCORD,
        "website" => WEBSITE,
        "appPage" => APP_PAGE,
        "github" => GITHUB,
        "issues" => ISSUES,
        _ => return None,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn links_follow_the_app_slug() {
        assert!(APP_PAGE.contains("DESIGN-SMART"));
        assert!(GITHUB.contains("DESIGN-SMART"));
        assert!(ISSUES.starts_with(GITHUB));
        for k in ["discord", "website", "appPage", "github", "issues"] {
            assert!(get(k).is_some_and(|u| u.starts_with("https://")), "{k}");
            assert_eq!(all()[k], get(k).unwrap());
        }
    }
}
