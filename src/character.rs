use serde::Serialize;
use crate::ability::AbilityScores;

#[derive(Debug, Serialize, Clone)]
pub struct Character {
    pub level: u32,
    pub class: String,
    pub species: String,
    pub ability_scores: AbilityScores,
}
