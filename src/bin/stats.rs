use dice_parser::{DiceExpr, RollSpec, Keep};
use glog_v2_character_generator::{CharacterGenerator};
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct StatArgs {
    // number of stats to roll
    #[arg(short, long, default_value_t = 6)]
    stats: u32,

    // number of dice to roll per stat
    #[arg(short, long, default_value_t = 3)]
    dice: u32,

    // number of faces per die
    #[arg(short, long, default_value_t = 6)]
    faces: u32,

    // how many low rolls should be ignored
    #[arg(short, long, default_value_t = 0)]
    lowest: u32,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {

    let args = StatArgs::parse();
    let mut expr = DiceExpr::Roll(RollSpec::new(args.dice, args.faces, Some(Keep::Highest(args.dice-args.lowest))));
    let result = CharacterGenerator::generate_score(&mut expr);

    println!("Roll {}d{} drop {} lowest: {}", args.dice, args.faces, args.lowest, result);

    Ok(())
}
