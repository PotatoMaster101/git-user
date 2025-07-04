use serde::{Deserialize, Serialize};

/// Represents a profile in the config.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Profile {
    /// Username.
    pub name: String,

    /// User email.
    pub email: String,

    /// User signing key.
    #[serde(rename = "signingKey", skip_serializing_if = "Option::is_none")]
    pub signing_key: Option<String>,

    /// SSH command to run when authenticating.
    #[serde(rename = "sshCommand", skip_serializing_if = "Option::is_none")]
    pub ssh_command: Option<String>,
}

impl Profile {
    /// Creates a new `Profile`.
    pub fn new(name: &str, email: &str, signing_key: Option<String>, ssh_command: Option<String>) -> Self {
        Self { name: name.into(), email: email.into(), signing_key, ssh_command }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_creates_profile() {
        let profile = Profile::new("", "", None, None);
        assert_eq!("", profile.name);
        assert_eq!("", profile.email);
        assert_eq!(None, profile.signing_key);
        assert_eq!(None, profile.ssh_command);

        let profile = Profile::new("abc", "def", Some("ghi".into()), Some("jkl".into()));
        assert_eq!("abc", profile.name);
        assert_eq!("def", profile.email);
        assert_eq!(Some("ghi".into()), profile.signing_key);
        assert_eq!(Some("jkl".into()), profile.ssh_command);
    }
}
