mod cli;
// mod app;
mod inner;
use inner::{Advanced, Basic};

fn main() {
    let cli = cli::cli().get_matches();

    let mut creatures = inner::Elements::new("creatures", include_str!("data/creatures"));
    let mut colors = inner::Elements::new("colors", include_str!("data/colors"));
    creatures.read_cwd_or_origin();
    colors.read_cwd_or_origin();
    let composition = inner::Composition::new(vec![&colors, &creatures]);

    match &cli.subcommand().unwrap() {
        ("generate", args) => {
            let generate_output = *args.get_one::<u8>("num").unwrap();
            let max_chars = *args.get_one::<u8>("max").unwrap();
            // let generate_output = args.value_of("num").unwrap().parse::<i32>().unwrap();
            for _ in 0..generate_output {
                let find_random = || {
                    for _ in 0..composition.count() {
                        let output = composition.random();
                        if output.len() <= max_chars as usize {
                            return Some(output);
                        }
                    }
                    None
                };
                println!(
                    "{}",
                    find_random().unwrap_or(format!(
                        "No combinations available of length: {}",
                        max_chars
                    ))
                )
            }
        }
        ("reset", _) => {
            creatures.write_to_home();
            colors.write_to_home();
        }
        ("stat", args) => match args.subcommand().unwrap() {
            ("count", args) => match args.get_one::<usize>("size") {
                Some(&size) => {
                    // let size = size.parse::<usize>().unwrap();
                    println!("creatures: {}", creatures.filter_on_len(size).len());
                    println!("colors: {}", colors.filter_on_len(size).len());
                }
                None => {
                    println!("creatures: {}", creatures.count());
                    println!("colors: {}", colors.count());
                    println!("combinations: {}", composition.count());
                }
            },
            _ => {}
        },
        ("creature", args) => match args.subcommand().unwrap() {
            ("add", args) => creatures.check_print_add(
                args.get_many::<&str>("creature")
                    .unwrap_or_default()
                    .map(Clone::clone)
                    .collect(),
            ),
            _ => {}
        },
        ("color", args) => match args.subcommand().unwrap() {
            ("add", args) => creatures.check_print_add(
                args.get_many::<&str>("creature")
                    .unwrap()
                    .map(Clone::clone)
                    .collect(),
            ),
            _ => {}
        },
        _ => {}
    }
}
