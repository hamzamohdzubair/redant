pub fn cli() -> clap::Command {
    clap::command!()
        .arg_required_else_help(true)
        .subcommand(
            clap::Command::new("generate")
                .alias("gen")
                .about("generate a combination of color and creature")
                .arg(
                    clap::arg!(num: -n --num [INT] "select number of outputs")
                        .default_value("1")
                        .value_parser(clap::value_parser!(u8).range(1..=100)),
                )
                .arg(
                    clap::arg!(max: -m --max [INT] "select max length of generated word")
                        .default_value("40")
                        .value_parser(clap::value_parser!(u8).range(1..=100)),
                ),
        )
        .subcommand(clap::Command::new("reset").about("reset local data"))
        .subcommand(
            clap::Command::new("stat")
                .about("print stats related to creatures and colors")
                .arg_required_else_help(true)
                .subcommand(
                    clap::Command::new("count")
                        .about("print the total counts")
                        .arg(
                            clap::arg!(size: -s --size [INT] "count words of given size")
                                .value_parser(clap::value_parser!(u8).range(1..=100)),
                        ),
                ),
        )
        .subcommand(
            clap::Command::new("creature")
                .about("manipulate local creature list")
                .arg_required_else_help(true)
                .subcommand(
                    clap::Command::new("add")
                        .about("add list of creatures to local db")
                        .arg(clap::arg!(creature: <CREATURES> ...)),
                ),
        )
        .subcommand(clap::Command::new("color").about("manipulate local color list"))
}
