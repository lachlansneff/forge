use std::env;

use cargo::Subcommand;

pub struct Args {
    all: Vec<String>,
    subcommand: Option<Subcommand>
}

impl Args {
    pub fn all(&self) -> &[String] {
        &self.all
    }

    pub fn subcommand(&self) -> Option<Subcommand> {
        self.subcommand
    }

    pub fn verbose(&self) -> bool {
        self.all
            .iter()
            .any(|a| a == "--verbose" || a == "-v" || a == "-vv")
    }

    pub fn version(&self) -> bool {
        self.all
            .iter()
            .any(|a| a == "--version" || a == "-V")
    }
}

pub fn args() -> Args {
    let all: Vec<String> = env::args().skip(1).collect();

    let sc = all
            .iter()
            .find(|arg| {
                !arg.starts_with("-")
            })
            .map(|arg| Subcommand::from(arg));
    

    Args {
        all: all,
        subcommand: sc,
    }
}