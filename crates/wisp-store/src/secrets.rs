//! OS keyring-backed secret storage for API keys.

use keyring::Entry;

const SERVICE: &str = "wisp";

/// A named secret (e.g. an API key) stored in the OS credential manager.
pub struct Secret;

impl Secret {
    pub fn set(name: &str, value: &str) -> anyhow::Result<()> {
        let entry = Entry::new(SERVICE, name)?;
        entry.set_password(value)?;
        Ok(())
    }

    pub fn get(name: &str) -> anyhow::Result<String> {
        let entry = Entry::new(SERVICE, name)?;
        Ok(entry.get_password()?)
    }

    pub fn delete(name: &str) -> anyhow::Result<()> {
        let entry = Entry::new(SERVICE, name)?;
        entry.delete_credential()?;
        Ok(())
    }
}
