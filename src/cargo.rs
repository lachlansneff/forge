
use std::process::Command;
use std::cell::RefCell;

pub struct Cargo {
    flags: RefCell<Vec<String>>,
    command: String,
}

pub enum Flavor {
    Cargo,
    Xargo,
}

impl Cargo {
    pub fn new(flavor: Flavor) -> Cargo {
        Cargo {
            flags: RefCell::new(Vec::new()),
            command: String::from(match flavor {
                Flavor::Cargo => "cargo",
                Flavor::Xargo => "xargo",
            }),
        }
    }

    pub fn add_flag(&self, s: &str) {
        self.flags.borrow_mut().push(String::from(s));
    }

    pub fn run(self) -> Result<(), ()> {
        self.run_env(("", ""))
    }

    pub fn run_env(self, env: (&str, &str)) -> Result<(), ()> {
        match Command::new(self.command)
                .args(self.flags.into_inner())
                .env(env.0, env.1)
                .status()
        {
            Ok(_) => Ok(()),
            Err(_) => Err(())
        }
    }
}

#[derive(Copy, Clone)]
pub enum Subcommand {
    Clean,
    Init,
    New,
    Build,
    Other,
}

impl Subcommand {
    pub fn from(sc: &str) -> Subcommand {
        match sc {
            "clean" => Subcommand::Clean,
            "init" => Subcommand::Init,
            "new" => Subcommand::New,
            "build" => Subcommand::Build,
            _ => Subcommand::Other,
        }
    }
}