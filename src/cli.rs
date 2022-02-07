lazy_static::lazy_static!{
    static ref DIGITS: regex::Regex = regex::Regex::new(r"\d+").unwrap();
}

pub fn cli() -> clap::App<'static> {
    clap::app_from_crate!()
        .setting(clap::AppSettings::SubcommandRequiredElseHelp)
        .subcommand(
            clap::App::new("generate")
            .about("generate a combination of color and creature")
            .arg(
                clap::arg!(num: -n --num [INT] "select number of outputs")
                .default_value("1")
                .validator_regex(&*DIGITS, "only numbers are allowed")
                .multiple_values(false)
            )
            .arg(
                clap::arg!(max: -m --max [INT] "select max length of generated word")
                .default_value("40")
                .validator_regex(&*DIGITS, "only numbers are allowed")
                .multiple_values(false)
            )
        )
        .subcommand(
            clap::App::new("reset")
            .about("reset local data")
        )
        .subcommand(
            clap::App::new("stat")
            .about("print stats related to creatures and colors")
            .setting(clap::AppSettings::SubcommandRequiredElseHelp)
            .subcommand(
                clap::App::new("count")
                .about("print the total counts")
                .arg(
                    clap::arg!(size: -s --size [INT] "count words of given size")
                )

            )
        )
        .subcommand(
            clap::App::new("creature")
            .about("manipulate local creature list")
            .setting(clap::AppSettings::SubcommandRequiredElseHelp)
            .subcommand(
                clap::App::new("add")
                .about("add list of creatures to local db")
                .arg(
                    clap::arg!(creature: <CREATURES> ...)
                )


                )
        )
        .subcommand(
            clap::App::new("color")
                .about("manipulate local color list")
        )
}