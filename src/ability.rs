use dice_parser::{DiceExpr, RollSpec, Keep};
use serde::Serialize;

#[derive(Debug, Serialize, Clone)]
pub struct AbilityScores {
    pub strength: u32,
    pub dexterity: u32,
    pub constitution: u32,
    pub intelligence: u32,
    pub wisdom: u32,
    pub charisma: u32,
}

pub fn generate_scores(dice: u32, faces: u32, lowest: u32) -> anyhow::Result<AbilityScores> {
    if lowest > dice {
        return Err(anyhow::anyhow!("lowest ({}) cannot exceed dice count ({})", lowest, dice));
    }

    let mut expr = DiceExpr::Roll(RollSpec::new(dice, faces, Some(Keep::Highest(dice - lowest))));
    Ok(AbilityScores {
        strength: roll(&mut expr)?,
        dexterity: roll(&mut expr)?,
        constitution: roll(&mut expr)?,
        intelligence: roll(&mut expr)?,
        wisdom: roll(&mut expr)?,
        charisma: roll(&mut expr)?,
    })
}

fn roll(expr: &mut DiceExpr) -> anyhow::Result<u32> {
    let result = expr.roll()
        .map_err(|e| anyhow::anyhow!("Dice roll failed: {}", e))?;
    Ok(result.total as u32)
}
