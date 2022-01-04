use rand::seq::IteratorRandom;
use clap::Parser;
use colored::*;

lazy_static::lazy_static!{
    static ref CWD: std::path::PathBuf = dirs::home_dir().unwrap().join(".redant");

}

mod cli;

fn main() {
    let cli = cli::Cli::parse();

    let creatures = std::fs::read_to_string(&*CWD.join("creatures"))
        .unwrap_or_else( |_|{
            std::fs::create_dir_all(&*CWD).unwrap();
            read_write_return(include_str!("data/creatures"), "creatures")
        });

    let colors = std::fs::read_to_string(&*CWD.join("colors"))
        .unwrap_or_else( |_|{
            std::fs::create_dir_all(&*CWD).unwrap();
            read_write_return(include_str!("data/colors"), "colors")
        });

    match &cli.command {
        cli::Commands::Reset {} => {
            std::fs::create_dir_all(&*CWD).unwrap();
            std::fs::write(&*CWD.join("creatures"), include_str!("data/creatures")).unwrap();
            std::fs::write(&*CWD.join("colors"), include_str!("data/colors")).unwrap();
        }

        cli::Commands::Generate { num, max} => {
            for _ in 0..num.unwrap_or(1) {
                for _ in 0..5000 {
                    let random_creature = creatures.lines().choose(&mut rand::thread_rng()).unwrap();
                    let random_color = colors.lines().choose(&mut rand::thread_rng()).unwrap();
                    let combination = format!("{}{}", random_color, random_creature);

                    if combination.len() <= max.unwrap_or(100) {
                        println!("{}{}", random_color, random_creature);
                        break;
                    }

                }
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
                cli::CreatureCommands::Add { inputs } => {
                    let new_colors = check_for_existence_then_add(inputs, &creatures);
                    std::fs::write(&*CWD.join("creatures"), new_colors).unwrap();
                }
            }
        }

        cli::Commands::Color { command} => {
            match &command {
                cli::ColorCommands::Add{ inputs} => {
                    let new_colors = check_for_existence_then_add(inputs, &colors);
                    std::fs::write(&*CWD.join("colors"), new_colors).unwrap();
                }
            }
        }
    }

}


fn check_for_existence_then_add(lst_inputs: &Vec<String>, gold_string: &str ) -> String {
    let mut return_string = gold_string.to_owned();
    for elem in lst_inputs.iter() {
        if return_string.lines().any(|oneline| oneline==elem) {
            print!("{} ", elem.red());
        } else {
            print!("{} ", elem.green());
            return_string.push_str(&format!("\n{}", &elem))
        }
    }
    println!(" ");
    return_string
}

fn read_write_return(in_file: &str, out_file: &str) -> String {
    std::fs::write(&*CWD.join(out_file), in_file).unwrap();
    in_file.to_owned()
}