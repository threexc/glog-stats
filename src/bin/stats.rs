use glog_v2_character_generator::{CharacterGenerator};
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct StatArgs {
    // number of stats to roll
    #[arg(short, long, default_value_t = 6)]
    stats: u8,

    // number of dice to roll per stat
    #[arg(short, long, default_value_t = 3)]
    dice: u8,

    // number of faces per die
    #[arg(short, long, default_value_t = 6)]
    faces: u8,

    // how many low rolls should be ignored
    #[arg(short, long, default_value_t = 0)]
    lowest: u8,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {

    let args = StatArgs::parse();
    let mut rng = rand::thread_rng();
    let result = CharacterGenerator::roll_ability_score(&mut rng, args.dice, args.faces, args.lowest);

    println!("Roll {}d{} drop {} lowest: {}", args.dice, args.faces, args.lowest, result);

    Ok(())
}
