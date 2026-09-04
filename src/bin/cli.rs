use glog_v2_character_generator::CharacterGenerator;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct CharacterArgs {
    // config file to use
    #[arg(long, default_value = "config.toml")]
    config: String,

    // level
    #[arg(short, long, default_value_t = 1)]
    level: u32,

    // number of dice for stats
    #[arg(short, long, default_value_t = 3)]
    dice: u32,

    // number of faces for dice
    #[arg(short, long, default_value_t = 6)]
    faces: u32,

    // how many characters to create
    #[arg(short, long, default_value_t = 1)]
    count: u32,

    // how many low rolls should be ignored
    #[arg(long, default_value_t = 0)]
    lowest: u32,
}

fn main() -> anyhow::Result<()> {
    println!("🎲 GLOG v2 Character Generator (CLI)");
    println!("================================");
    
    let args = CharacterArgs::parse();
    if args.lowest > args.dice {
        return Err(anyhow::anyhow!("Can't keep negative dice"));
    }

    // Initialize the character generator
    let generator = CharacterGenerator::new(&args.config)?;
    
    // Generate characters
    let characters = generator.generate_characters(args.level, args.count, args.dice, args.faces, args.lowest)?;
    
    // Display characters
    for (i, character) in characters.iter().enumerate() {
        println!("\nCharacter {}:", i + 1);
        println!("Level: {}", character.level);
        println!("Species: {}", character.species);
        println!("Class: {}", character.class);
        println!("Ability Scores:");
        println!("  Strength: {}", character.ability_scores.strength);
        println!("  Dexterity: {}", character.ability_scores.dexterity);
        println!("  Constitution: {}", character.ability_scores.constitution);
        println!("  Intelligence: {}", character.ability_scores.intelligence);
        println!("  Wisdom: {}", character.ability_scores.wisdom);
        println!("  Charisma: {}", character.ability_scores.charisma);
    }
    
    // Save to file
    println!("\n{} character(s) generated successfully!", args.count);
    println!("Characters saved to character.txt");
    
    Ok(())
}
