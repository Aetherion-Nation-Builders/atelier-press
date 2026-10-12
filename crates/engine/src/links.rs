//! The Nation and project links (Help menu, About, start screen, CLI, MCP). The keys keep their
//! upstream names (`discord` is the Nation) so commands and scripts stay the same.

use serde_json::{Value, json};

/// The app's slug on the Nation site and its repository name (`atelier-press`).
pub const APP: &str = "press";
/// The Aetherion Nation (the `help.discord` command).
pub const DISCORD: &str = "https://www.aetherionnnation.com";
/// Atelier, the Nation's creative suite.
pub const WEBSITE: &str = "https://www.aetherionnnation.com/atelier";
/// Atelier Press's page on the Nation site.
pub const APP_PAGE: &str = "https://www.aetherionnnation.com/atelier/press";
/// Atelier Press's source repository.
pub const GITHUB: &str = "https://github.com/Aetherion-Nation-Builders/atelier-press";
/// Where to report bugs and request features.
pub const ISSUES: &str = "https://github.com/Aetherion-Nation-Builders/atelier-press/issues";

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
    fn links_follow_the_atelier_scheme() {
        assert!(WEBSITE.starts_with(DISCORD));
        assert_eq!(APP_PAGE, format!("{WEBSITE}/{APP}"));
        assert!(GITHUB.ends_with(&format!("/Aetherion-Nation-Builders/atelier-{APP}")));
        assert!(ISSUES.starts_with(GITHUB));
        for k in ["discord", "website", "appPage", "github", "issues"] {
            assert!(get(k).is_some_and(|u| u.starts_with("https://")), "{k}");
            assert_eq!(all()[k], get(k).unwrap());
        }
    }
}
