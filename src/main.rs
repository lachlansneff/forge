
//! Application for building bare-metal applications that use `metal`.


extern crate toml;
extern crate rustc_version;
#[macro_use]
extern crate serde_derive;
extern crate serde;

mod log;
mod cargo;
mod cli;
mod create;
mod build;
mod config;

use rustc_version::{version_meta, Channel};
use cargo::Subcommand;
use create::create_project;

fn main() {
    match run() {
        Ok(_) => {},
        Err(_) => {
            log::fail("Something went wrong :(");
        }
    }
}

fn run() -> Result<(), ()> {
    let args = cli::args();

    match args.subcommand() {
        Some(sc) => {
            match sc {
                Subcommand::New | Subcommand::Init => {
                    if let Some(path) = args.all().iter().nth(1) {
                        let new = match sc {
                            Subcommand::New => true,
                            Subcommand::Init => false,
                            _ => false
                        };
                        // couldn't figure out another way of doing this
                        let path = path.clone();

                        create_project(path, new).unwrap();
                    }
                },
                Subcommand::Build => {
                    match version_meta().expect("Couldn\'t find rustc version").channel {
                        Channel::Nightly => {
                            // do stuff
                            build::build();
                        },
                        _ => {
                            log::important("Current channel must be Nightly. Please switch to that.");
                        }
                    }
                }
                _ => {}
            }


            Ok(())
        },
        None => Err(()),
    }
}