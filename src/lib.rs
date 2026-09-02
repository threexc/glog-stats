use dice_parser::{DiceExpr, RollSpec, Keep};
use rand::Rng;
use serde::{Deserialize, Serialize};
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

#[derive(Debug, Serialize, Clone)]
pub struct Character {
    pub level: u32,
    pub class: String,
    pub species: String,
    pub ability_scores: AbilityScores,
}

#[derive(Debug, Serialize, Clone)]
pub struct AbilityScores {
    pub strength: u32,
    pub dexterity: u32,
    pub constitution: u32,
    pub intelligence: u32,
    pub wisdom: u32,
    pub charisma: u32,
}

pub struct CharacterGenerator {
    config: Config,
}

impl CharacterGenerator {
    pub fn new(config_path: &str) -> anyhow::Result<Self> {
        let config = Self::load_config(config_path)?;
        
        Ok(Self { config })
    }
    
    pub fn from_config(config: Config) -> Self {
        Self { config }
    }
    
    pub fn generate_character(&self, level: u32, dice: u32, faces: u32, lowest: u32) -> anyhow::Result<Character> {
        if level < 1 || level > 10 {
            return Err(anyhow::anyhow!("Level must be between 1 and 10"));
        }
        
        let mut rng = rand::thread_rng();
        let species = self.config.species[rng.gen_range(0..self.config.species.len())].clone();
        let mut class = self.config.classes[rng.gen_range(0..self.config.classes.len())].clone();

        if class.name == "Wizard" {
            let archetype = &self.config.wizard_archetypes[rng.gen_range(0..self.config.wizard_archetypes.len())];
            class.name = format!("Wizard ({})", archetype.name);
        }

        let ability_scores = Self::generate_scores(dice, faces, lowest);

        Ok(Character {
            level,
            class: class.name,
            species: species.name,
            ability_scores,
        })
    }
    
    pub fn generate_characters(&self, count: u32, level: u32, dice: u32, faces: u32, lowest: u32) -> anyhow::Result<Vec<Character>> {
        if count < 1 {
            return Err(anyhow::anyhow!("Must generate at least 1 character"));
        }
        
        if count > 100 {
            return Err(anyhow::anyhow!("Cannot generate more than 100 characters at once"));
        }
        
        let mut characters = Vec::new();
        
        for _ in 0..count {
            characters.push(self.generate_character(level, dice, faces, lowest)?);
        }
        
        Ok(characters)
    }
    
    pub fn get_config(&self) -> &Config {
        &self.config
    }

    fn load_config(filename: &str) -> anyhow::Result<Config> {
        let content = fs::read_to_string(filename)
            .map_err(|_| anyhow::anyhow!("Could not read config file: {}", filename))?;
        
        let config: Config = toml::from_str(&content)
            .map_err(|e| anyhow::anyhow!("Invalid config file format: {}", e))?;
        
        Self::validate_config(&config)?;
        
        Ok(config)
    }
    
    fn validate_config(config: &Config) -> anyhow::Result<()> {
        if config.species.is_empty() {
            return Err(anyhow::anyhow!("Config file must contain at least one species"));
        }

        if config.classes.is_empty() {
            return Err(anyhow::anyhow!("Config file must contain at least one class"));
        }
        
        if config.wizard_archetypes.is_empty() {
            return Err(anyhow::anyhow!("Config file must contain at least one wizard archetype"));
        }
        
        Ok(())
    }
    
    pub fn generate_score(expr: &mut DiceExpr) -> u32 {
        expr.roll().unwrap().total as u32
    }

    pub fn generate_scores(dice: u32, faces: u32, lowest: u32) -> AbilityScores {
        let mut expr = DiceExpr::Roll(RollSpec::new(dice, faces, Some(Keep::Highest(dice-lowest))));
        AbilityScores {
            strength: Self::generate_score(&mut expr),
            dexterity: Self::generate_score(&mut expr),
            constitution: Self::generate_score(&mut expr),
            intelligence: Self::generate_score(&mut expr),
            wisdom: Self::generate_score(&mut expr),
            charisma: Self::generate_score(&mut expr),
        }
    }
}

// Utility functions for file operations
pub fn save_characters_to_file(characters: &[Character], level: u32, count: u32) -> anyhow::Result<String> {
    let filename = format!("characters_level_{}_count_{}.toml", level, count);
    
    // Create a wrapper struct to hold all characters
    #[derive(Serialize)]
    struct CharacterCollection<'a> {
        characters: &'a [Character],
    }
    
    let collection = CharacterCollection { characters };
    let content = toml::to_string_pretty(&collection)?;
    fs::write(&filename, content)?;
    
    Ok(filename)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    fn create_test_config() -> Config {
        Config {
            species: vec!["Human".to_string(), "Elf".to_string()],
            classes: vec!["Fighter".to_string(), "Wizard".to_string()],
            wizard_archetypes: vec!["Necromancer".to_string(), "Pyromancer".to_string()],
        }
    }
    
    #[test]
    fn test_character_generation() {
        let config = create_test_config();
        let generator = CharacterGenerator::from_config(config);
        
        let character = generator.generate_character(5, 3, 6, 0).unwrap();
        
        assert_eq!(character.level, 5);
        assert!(!character.species.is_empty());
        assert!(!character.class.is_empty());
        assert!(character.ability_scores.strength >= 3 && character.ability_scores.strength <= 18);
    }
    
    #[test]
    fn test_multiple_character_generation() {
        let config = create_test_config();
        let generator = CharacterGenerator::from_config(config);
        
        let characters = generator.generate_characters(10, 3, 4, 6, 1).unwrap();
        
        assert_eq!(characters.len(), 10);
        assert!(characters.iter().all(|c| c.level == 3));
    }
    
    #[test]
    fn test_invalid_level() {
        let config = create_test_config();
        let generator = CharacterGenerator::from_config(config);
        
        assert!(generator.generate_character(14, 3, 6, 0).is_err());
        assert!(generator.generate_character(21, 4, 6, 1).is_err());
    }
    
    #[test]
    fn test_wizard_archetype() {
        let config = create_test_config();
        let generator = CharacterGenerator::from_config(config);
        
        // Generate many characters to eventually get a wizard
        for _ in 0..100 {
            let character = generator.generate_character(1, 3, 6, 0).unwrap();
            if character.class.starts_with("Wizard") {
                assert!(character.class.contains("("));
                assert!(character.class.contains(")"));
                break;
            }
        }
    }
}
