use rand::seq::IteratorRandom;
use clap::Parser;

mod cli;

fn main() {
    let cli = cli::Cli::parse();

    let creatures = include_str!("data/creatures");
    let colors = include_str!("data/colors");

    match &cli.command {
        cli::Commands::Generate { num} => {
            for _ in 0..num.unwrap_or(1) {
                let random_creature = creatures.lines().choose(&mut rand::thread_rng()).unwrap();
                let random_color = colors.lines().choose(&mut rand::thread_rng()).unwrap();
                println!("{}{}", random_color, random_creature)
            }

        }
        cli::Commands::Stat { command } => {
            match &command {
                cli::StatCommands::Count{} => {
                    let creatures_count = creatures.lines().count();
                    let colors_count = colors.lines().count();
                    println!("creatures: {}", creatures_count);
                    println!("colors: {}", colors_count);
                    println!("combinations: {}", creatures_count * colors_count)

                }
            }
        }
    }


}
