use std::collections::HashMap;
use std::fmt::Display;
use std::str::FromStr;
use serde::{Deserialize, Serialize};
use crate::profile::Profile;
use crate::error::AppError;

/// Represents a user config.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Config {
    /// Collection of config profiles.
    profiles: HashMap<String, Profile>,
}

impl Display for Config {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let json = serde_json::to_string(&self).map_err(|_| std::fmt::Error)?;
        write!(f, "{json}")
    }
}

impl FromStr for Config {
    type Err = AppError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(serde_json::from_str::<Self>(s)?)
    }
}

impl Config {
    /// Adds or updates a new profile.
    pub fn insert(&mut self, profile_name: &str, profile: &Profile) -> Option<Profile> {
        self.profiles.insert(profile_name.into(), profile.clone())
    }

    /// Deletes an profile.
    pub fn remove(&mut self, profile_name: &str) -> Option<Profile> {
        self.profiles.remove(profile_name)
    }

    /// Gets a specific profile.
    pub fn get(&self, profile_name: &str) -> Option<&Profile> {
        self.profiles.get(profile_name)
    }

    /// Gets a profiles iterator.
    pub fn iter(&self) -> impl Iterator<Item = (&String, &Profile)> {
        self.profiles.iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fmt_writes_json() {
        let mut config = Config::default();
        assert_eq!(r#"{"profiles":{}}"#, format!("{config}"));

        config.insert("test", &Profile::new("abc", "def", None, None));
        assert_eq!(r#"{"profiles":{"test":{"name":"abc","email":"def"}}}"#, format!("{config}"));
    }

    #[test]
    fn from_str_reads_json() {
        let config = Config::from_str(r#"{"profiles":{"test":{"name":"abc","email":"def"}}}"#).unwrap();
        assert_eq!(1, config.profiles.len());
        assert_eq!("abc", config.profiles["test"].name);
        assert_eq!("def", config.profiles["test"].email);
        assert_eq!(None, config.profiles["test"].signing_key);
        assert_eq!(None, config.profiles["test"].ssh_command);

        let config = Config::from_str(r#"{"profiles":{}}"#).unwrap();
        assert_eq!(0, config.profiles.len());

        let config = Config::from_str("invalid");
        assert!(config.is_err());
    }

    #[test]
    fn insert_inserts_profile() {
        let mut config = Config::default();
        config.insert("test", &Profile::new("abc", "def", None, None));

        assert_eq!(1, config.profiles.len());
        assert_eq!("abc", config.profiles["test"].name);
        assert_eq!("def", config.profiles["test"].email);
        assert_eq!(None, config.profiles["test"].signing_key);
        assert_eq!(None, config.profiles["test"].ssh_command);
    }

    #[test]
    fn remove_removes_profile() {
        let mut config = Config::default();
        config.insert("test", &Profile::new("abc", "def", None, None));

        config.remove("test");
        assert_eq!(0, config.profiles.len());

        config.remove("test");
        assert_eq!(0, config.profiles.len());
    }

    #[test]
    fn get_gets_profile() {
        let mut config = Config::default();
        config.insert("test", &Profile::new("abc", "def", None, None));

        let profile = config.get("test");
        assert_eq!("abc", profile.unwrap().name);
        assert_eq!("def", profile.unwrap().email);
        assert_eq!(None, profile.unwrap().signing_key);
        assert_eq!(None, profile.unwrap().ssh_command);
    }

    #[test]
    fn iter_gets_iterator() {
        let mut config = Config::default();
        config.insert("test1", &Profile::new("a", "b", None, None));
        config.insert("test2", &Profile::new("aa", "bb", Some("cc".into()), None));

        let profiles: HashMap<String, Profile> = config.iter().map(|(k, v)| (k.clone(), v.clone())).collect();
        assert_eq!(2, profiles.len());
        assert_eq!("a", profiles["test1"].name);
        assert_eq!("b", profiles["test1"].email);
        assert_eq!(None, profiles["test1"].signing_key);
        assert_eq!(None, profiles["test1"].ssh_command);
        assert_eq!("aa", profiles["test2"].name);
        assert_eq!("bb", profiles["test2"].email);
        assert_eq!(Some("cc".into()), profiles["test2"].signing_key);
        assert_eq!(None, profiles["test2"].ssh_command);
    }
}
