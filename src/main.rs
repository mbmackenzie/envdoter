extern crate envdoter;

use envdoter::db::Database;
use envdoter::{create_env_file, sort_env_file};
use std::path::{Path, PathBuf};
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "envdoter")]
struct Opt {
    /// Command to execute
    #[structopt(subcommand)]
    cmd: Command,
}

#[derive(StructOpt, Debug)]
enum Command {
    /// Create a new .env file
    Init {
        /// Path to the .env file, defaults to .env
        #[structopt(short, long, default_value = ".env")]
        path: String,
    },

    /// List all variable names
    Ls {
        /// Prefix to filter the variables
        #[structopt(short, long)]
        prefix: Option<String>,
    },

    /// Generate a new .env file using a template
    Generate {
        /// Template file to use
        #[structopt(long, default_value = ".env-sample")]
        from: String,
    },

    /// Add a new variable to the database
    Add {
        /// Name of the variable
        name: String,
        /// Value of the variable
        value: Option<String>,
    },

    /// Get a value from the database
    Get {
        /// Name of the variable
        name: String,
    },

    Sort {
        /// Path to the .env file, defaults to .env
        #[structopt(short, long, default_value = ".env")]
        path: String,
    },

    /// include a new variable to the .env file
    Include {
        /// Name of the variable
        name: String,
        /// Value of the variable
        value: Option<String>,
    },
}

fn main() {
    let db_path: PathBuf;

    if let Some(home_dir) = dirs::home_dir() {
        db_path = home_dir.join(".envdoter").join("db");
    } else {
        std::process::exit(1);
    }

    let db = Database::new(Path::new(db_path.as_path())).unwrap();

    let opt = Opt::from_args();
    match opt.cmd {
        Command::Init { path } => {
            let new_env = create_env_file(Path::new(path.as_str()));
            if new_env.is_err() {
                if new_env.unwrap_err().kind() == std::io::ErrorKind::AlreadyExists {
                    println!("File already exists. Exiting.");
                } else {
                    println!("Error creating file. Exiting.");
                }
            }
        }
        Command::Ls { prefix } => {
            let keys = db.read_keys();

            match prefix {
                Some(prefix) => {
                    for key in keys {
                        if key.starts_with(&prefix) {
                            println!("{}", key);
                        }
                    }
                }
                None => {
                    for key in keys {
                        println!("{}", key);
                    }
                }
            }
        }
        Command::Add { name, value } => match value {
            Some(value) => {
                db.write_value(&name, &value);
            }
            None => {
                if name.contains("=") {
                    let split: Vec<&str> = name.split("=").collect();
                    let name = split[0];
                    let new_value = split[1];
                    db.write_value(name, new_value);
                } else {
                    println!("No value provided. Exiting.");
                    std::process::exit(1);
                }
            }
        },
        Command::Get { name } => {
            let value = db.read_value(&name);
            println!("{}", value);
        }
        Command::Generate { from } => {
            println!("Generating a new .env file using template {:?}", from);
        }
        Command::Sort { path } => {
            match sort_env_file(Path::new(path.as_str())) {
                Ok(_) => {}
                Err(_) => println!("Error sorting file"),
            };
        }
        Command::Include { name, value } => {
            println!("Including a new variable {} with value {:?}", name, value);
        }
        _ => println!("Not implemented"),
    }
}
