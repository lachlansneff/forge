
use cargo::{Cargo, Flavor};
use config::Config;
use std::fs::{create_dir_all, OpenOptions, File};
use std::io::Write;
use std::collections::HashMap;

// A macro to easily create and populate hash maps
macro_rules! hashmap {
    { $($key:expr => $value:expr),+ } => {
        {
            let mut m = HashMap::new();
            $(
                m.insert($key, $value);
            )+
            m
        }
    };
    ($hm:ident, { $($key:expr => $value:expr),+ } ) => (
        {
            $(
                $hm.insert($key, $value);
            )+
        }
    );
}

pub fn create_project(path: String, new: bool) -> Result<(), ()> {
    let xargo = Cargo::new(Flavor::Xargo);
    // add new|init flag
    xargo.add_flag(match new {
        true => "new",
        false => "init"
    });
    xargo.add_flag(&path);

    xargo.run()?;

    mod_cargo_toml(&path);
    place_files(&path);

    Ok(())

}

fn mod_cargo_toml(path: &str) {
    let mut file = OpenOptions::new()
            .append(true)
            .open(format!("{}/Cargo.toml", path))
            .unwrap();
    
    write!(file, "
[lib]
crate-type = [\"staticlib\"]

[dependencies.metal]
git = \"https://github.com/metal-os/metal\"

    ").unwrap();
}

fn place_files(path: &str) {

    let arches = hashmap!{
        "x86_64" => hashmap!{
            "grub.cfg"  => include_str!("arch/x86_64/grub.cfg"),
            "layout.ld" => include_str!("arch/x86_64/layout.ld"),
            "x86_64-unknown-none.json" => include_str!("arch/x86_64/x86_64-unknown-none.json")
        }
    };

    let config = Config::new(Some("x86_64"));

    config.write(&format!("{}/{}", path, "Forge.toml"));

    for (arch, files) in &arches {
        let dir = format!("{}/arch/{}", path, arch);
        create_dir_all(dir.clone()).unwrap();

        for (filename, data) in files {
            File::create(format!("{}/{}", dir, filename))
                .unwrap()
                .write_all(data.as_bytes())
                .unwrap();
        }
    }

    // replace lib.rs
    let mut lib_rs = OpenOptions::new()
                    .write(true)
                    .open(format!("{}/src/lib.rs", path))
                    .unwrap();

    write!(lib_rs, "
#![no_std]
#![feature(asm)]

#[macro_use]
extern crate metal;

metal!({{
    // put code here for now
    loop {{ }}
}});
    ").unwrap();
}