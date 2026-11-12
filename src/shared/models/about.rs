use dioxus::fullstack::serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct About {
    pub branch: String,
    pub commit_hash_full: String,
    pub commit_hash_short: String,
    pub dirty: bool,
    pub build_date: String,
    pub tag: String,
    pub dependencies: Vec<DependencyInfo>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct DependencyInfo {
    pub name: String,
    pub version: String,
    pub url: String,
}

#[cfg(feature = "server")]
impl About {
    pub fn get() -> About {
        use crate::backend::database::db::{DB, sqlite_version};
        use serde_json::Value;

        #[cfg(debug_assertions)]
        const EMBEDDED: &str = include_str!("../../../assets/info-debug.json");

        #[cfg(not(debug_assertions))] // Created by build.rs
        const EMBEDDED: &str = include_str!("../../../assets/info-release.json");

        const FALLBACK: &str = "MISSING";
        let json: Value = serde_json::from_str(EMBEDDED).unwrap_or(Value::Null);

        let mut dependencies = vec![DependencyInfo {
            name: "SQLite".to_string(),
            version: DB
                .with(|db| sqlite_version(db))
                .unwrap_or(FALLBACK.to_string()),
            url: "https://www.sqlite.org/".to_string(),
        }];
        dependencies.extend(
            json["dependency-info"]
                .as_array()
                .unwrap_or(&vec![])
                .iter()
                .map(|element| DependencyInfo {
                    name: element["name"].as_str().unwrap_or(FALLBACK).to_string(),
                    version: element["version"].as_str().unwrap_or(FALLBACK).to_string(),
                    url: element["url"].as_str().unwrap_or(FALLBACK).to_string(),
                }),
        );

        About {
            branch: json["branch"].as_str().unwrap_or(FALLBACK).to_string(),
            commit_hash_full: json["commit-hash-full"]
                .as_str()
                .unwrap_or(FALLBACK)
                .to_string(),
            commit_hash_short: json["commit-hash-short"]
                .as_str()
                .unwrap_or(FALLBACK)
                .to_string(),
            dirty: json["dirty"].as_bool().unwrap_or(true),
            build_date: json["build-date"].as_str().unwrap_or(FALLBACK).to_string(),
            tag: json["tag"].as_str().unwrap_or(FALLBACK).to_string(),
            dependencies,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::shared::models::about::About;

    #[test]
    fn parse_embedded_info() {
        let about = About::get();
        // print!("{:?}", about);
        assert_ne!(about.branch, "MISSING");
        assert!(!about.dependencies.is_empty());
    }
}
