#[derive(clap::Parser)]
#[clap(author, version, about)]
#[clap(setting(clap::AppSettings::SubcommandRequiredElseHelp))]
pub struct Cli {

    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(clap::Subcommand)]
pub enum Commands {

    /// generate a combination of color and creature
    Generate {
        
        /// number of generations
        #[clap(short='n', long)]
        num: Option<usize>,

        /// max length of generated word
        #[clap(short='m', long)]
        max: Option<usize>,


    },

    /// reset local data
    Reset { },

    /// print stats related to creatures and colors
    #[clap(setting(clap::AppSettings::SubcommandRequiredElseHelp))]
    Stat {

        #[clap(subcommand)]
        command: StatCommands
    },

    /// manipulate creature list
    #[clap(setting(clap::AppSettings::SubcommandRequiredElseHelp))]
    Creature {

        #[clap(subcommand)]
        command: CreatureCommands
    },

    /// manipulate color list
    #[clap(setting(clap::AppSettings::SubcommandRequiredElseHelp))]
    Color {

        #[clap(subcommand)]
        command: ColorCommands
    },
}


#[derive(clap::Subcommand)]
pub enum StatCommands {

    /// print counts of creatures, colors and combinations
    Count {}

}

#[derive(clap::Subcommand)]
pub enum CreatureCommands {

    /// add multiple creatures
    Add {

        inputs: Vec<String>,
    }

}

#[derive(clap::Subcommand)]
pub enum ColorCommands {

    /// add multiple colors
    Add {
        inputs: Vec<String>,
    }

}