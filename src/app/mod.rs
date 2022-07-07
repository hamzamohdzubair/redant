mod inner;
use inner::{Basic, Advanced};

pub struct App<'a> {
    creatures: inner::Elements,
    colors: inner::Elements,
    composition: inner::Composition<'a>,
}

impl App<'_> {
    fn new(cli: clap::ArgMatches) -> Self {
        let mut creatures = inner::Elements::new("creatures", include_str!("../data/creatures"));
        let mut colors = inner::Elements::new("colors", include_str!("../data/colors"));
        creatures.read_cwd_or_origin();
        colors.read_cwd_or_origin();
        let composition = inner::Composition::new(vec![&colors, &creatures]);
        Self {
            creatures: creatures,
            colors: colors,
            composition: composition,
        }

    }
}

// pub trait