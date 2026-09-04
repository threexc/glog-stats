pub mod ability;
pub mod character;
pub mod config;

use rand::Rng;
use std::fs;
use serde::Serialize;

use crate::character::Character;
use crate::config::Config;

pub struct CharacterGenerator {
    config: Config,
}

impl CharacterGenerator {
    pub fn new(config_path: &str) -> anyhow::Result<Self> {
        let config = Config::load(config_path)?;
        Ok(Self { config })
    }

    pub fn from_config(config: Config) -> Self {
        Self { config }
    }

    pub fn generate_character(
        &self,
        level: u32,
        dice: u32,
        faces: u32,
        lowest: u32,
    ) -> anyhow::Result<Character> {
        if !(1..=10).contains(&level) {
            return Err(anyhow::anyhow!("Level must be between 1 and 10"));
        }

        let mut rng = rand::thread_rng();

        let species = &self.config.species[rng.gen_range(0..self.config.species.len())];
        let class = &self.config.classes[rng.gen_range(0..self.config.classes.len())];

        let class_str = if class.name == "Wizard" {
            let archetype = &self.config.wizard_archetypes
                [rng.gen_range(0..self.config.wizard_archetypes.len())];
            format!("Wizard ({})", archetype.name)
        } else {
            class.name.clone()
        };

        let ability_scores = ability::generate_scores(dice, faces, lowest)?;

        Ok(Character {
            level,
            class: class_str,
            species: species.name.clone(),
            ability_scores,
        })
    }

    pub fn generate_characters(
        &self,
        count: u32,
        level: u32,
        dice: u32,
        faces: u32,
        lowest: u32,
    ) -> anyhow::Result<Vec<Character>> {
        if count == 0 {
            return Err(anyhow::anyhow!("Must generate at least 1 character"));
        }
        if count > 100 {
            return Err(anyhow::anyhow!("Cannot generate more than 100 characters at once"));
        }

        (0..count)
            .map(|_| self.generate_character(level, dice, faces, lowest))
            .collect()
    }

    pub fn get_config(&self) -> &Config {
        &self.config
    }
}

pub fn save_characters_to_file(
    characters: &[Character],
    path: &str,
) -> anyhow::Result<()> {
    #[derive(Serialize)]
    struct CharacterCollection<'a> {
        characters: &'a [Character],
    }

    let collection = CharacterCollection { characters };
    let content = toml::to_string_pretty(&collection)?;
    fs::write(path, content)?;
    Ok(())
}
