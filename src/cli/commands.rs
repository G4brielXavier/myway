use clap::Subcommand;

#[derive(Subcommand, Debug)]
pub enum Commands {

    /// Show informations about MYWAY
    Hey,


    /// Creates a new project and add to WAY
    Create,


    /// Show all projects added on WAY
    Way {
        #[arg(short, long)]
        oneline: bool,

        #[arg(short, long)]
        complex: bool,

        #[arg(short, long)]
        uuid: Option<String>,

        #[arg(short, long)]
        name: Option<String>
    },


    /// Remove a project from WAY
    Giveup {

        #[arg(short, long)]
        uuid: Option<String>,

        #[arg(short, long)]
        name: Option<String>

    },


    /// Define a project as "Done" by UUID
    Finish {

        #[arg(short, long)]
        uuid: Option<String>,

        #[arg(short, long)]
        name: Option<String>,

    },


    /// Edit an existent project by UUID
    Edit {

        #[arg(short, long)]
        uuid: Option<String>,

        #[arg(short, long)]
        name: Option<String>,

    },
    


    Status {
        
        #[arg(short, long)]
        uuid: Option<String>,

        #[arg(short, long)]
        name: Option<String>,
        
    },


    

    Version {

        #[arg(short, long)]
        uuid: Option<String>,

        #[arg(short, long)]
        name: Option<String>,

        #[arg(short, long)]
        list: bool,

        #[arg(short, long)]
        add: bool,

    },



    /// Show a report of all stacks users
    Stacks,



    Graveyard {

        #[arg(short, long)]
        uuid: Option<String>,

        #[arg(short, long)]
        name: Option<String>,

        #[arg(short, long)]
        list: bool,

        #[arg(short, long)]
        kill: bool,

        #[arg(short, long)]
        exject: bool
    
    },



    List,

}