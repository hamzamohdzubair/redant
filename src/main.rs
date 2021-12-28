use rand::seq::IteratorRandom;
use clap::Parser;
use colored::*;

mod cli;

fn main() {
    let cli = cli::Cli::parse();

    // std::fs::create_dir_all("redant.data").unwrap();
    // std::fs::write("redant.data/creatures", include_str!("data/creatures")).unwrap();
    // std::fs::write("redant.data/colors", include_str!("data/colors")).unwrap();
    // let creatures = std::fs::read_to_string("redant.data/creatures").unwrap();
    // let colors = std::fs::read_to_string("redant.data/colors").unwrap();

    let creatures = std::fs::read_to_string("redant.data/creatures")
        .unwrap_or_else( |_|{
            std::fs::create_dir_all("redant.data").unwrap();
            read_write_return(include_str!("data/creatures"), "creatures")
        });

    let colors = std::fs::read_to_string("redant.data/colors")
        .unwrap_or_else( |_|{
            std::fs::create_dir_all("redant.data").unwrap();
            read_write_return(include_str!("data/colors"), "colors")
        });

    match &cli.command {
        cli::Commands::Reset {} => {
            std::fs::create_dir_all("redant.data").unwrap();
            std::fs::write("redant.data/creatures", include_str!("data/creatures")).unwrap();
            std::fs::write("redant.data/colors", include_str!("data/colors")).unwrap();
        }

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
                cli::CreatureCommands::Add { inputs } => {
                    let new_colors = check_for_existence_then_add(inputs, &creatures);
                    std::fs::write("redant.data/creatures", new_colors).unwrap();
                }
            }
        }

        cli::Commands::Color { command} => {
            match &command {
                cli::ColorCommands::Add{ inputs} => {
                    let new_colors = check_for_existence_then_add(inputs, &colors);
                    std::fs::write("redant.data/colors", new_colors).unwrap();
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
    std::fs::write(format!("{}/{}", "redant.data", out_file), in_file).unwrap();
    in_file.to_owned()
}