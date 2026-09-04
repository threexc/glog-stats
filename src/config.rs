use serde::Deserialize;
use std::fs;

#[derive(Debug, Deserialize, Clone)]
pub struct Species {
    pub name: String,
    pub reroll: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Class {
    pub name: String,
    pub has_md: bool,
}

#[derive(Debug, Deserialize, Clone)]
pub struct WizardArchetype {
    pub name: String,
    pub school_type: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub species: Vec<Species>,
    pub classes: Vec<Class>,
    pub wizard_archetypes: Vec<WizardArchetype>,
}

impl Config {
    pub fn load(filename: &str) -> anyhow::Result<Self> {
        let content = fs::read_to_string(filename)
            .map_err(|e| anyhow::anyhow!("Could not read config file {}: {}", filename, e))?;

        let config: Config = toml::from_str(&content)
            .map_err(|e| anyhow::anyhow!("Invalid config file format: {}", e))?;

        Self::validate(&config)?;
        Ok(config)
    }

    fn validate(config: &Config) -> anyhow::Result<()> {
        if config.species.is_empty() {
            return Err(anyhow::anyhow!("Config must contain at least one species"));
        }
        if config.classes.is_empty() {
            return Err(anyhow::anyhow!("Config must contain at least one class"));
        }
        if config.wizard_archetypes.is_empty() {
            return Err(anyhow::anyhow!("Config must contain at least one wizard archetype"));
        }
        Ok(())
    }
}
