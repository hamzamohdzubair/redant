use rand::seq::IteratorRandom;
use clap::Parser;
use colored::*;

mod cli;

fn main() {
    let cli = cli::Cli::parse();

    let creatures = std::fs::read_to_string("redant.data/creatures")
        .unwrap_or_else( |_|{
            include_str!("data/creatures").to_owned()
        });

    let colors = std::fs::read_to_string("redant.data/colors")
        .unwrap_or_else( |_|{
            include_str!("data/colors").to_owned()
        });

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

        cli::Commands::Creature { command} => {
            match &command {
                cli::CreatureCommands::Add{names} => {
                    for name in names.iter() {
                        if creatures.lines().any(|oneline| oneline==name) {
                            print!("{} ", name.red());
                        } else {
                            print!("{} ", name.green());
                            let mut owned_creatures = creatures.to_owned();
                            owned_creatures.push_str(name);
                            std::fs::create_dir_all("redant.data").unwrap();
                            std::fs::write("redant.data/creatures", owned_creatures).unwrap();
                        }
                    }
                    println!(" ");
                }
            }
        }

        cli::Commands::Color { command} => {
            match &command {
                cli::ColorCommands::Add{} => {
                    println!("Running add command in color");
                }
            }
        }
    }


}
