use config::Config;
use cargo::{Cargo, Flavor};
use std::process::Command;
use std::fs::{create_dir_all, remove_dir_all, copy, File};
use std::io::Read;
use toml::from_str;

#[derive(Deserialize)]
struct CargoToml {
    package: CargoPackage,
}

#[derive(Deserialize)]
struct CargoPackage {
    name: String
}

pub fn build() {
    let config = Config::open("Forge.toml").unwrap();

    let target = match config.target() {
        Some(t) => t,
        None => return,
    };

    let target_dir = format!("arch/{}/", target);

    let xargo = Cargo::new(Flavor::Xargo);
    xargo.add_flag("build");
    xargo.add_flag("--release");
    xargo.add_flag("--target");
    xargo.add_flag(&format!("{}-unknown-none", target));
    xargo.run_env(
        ("RUST_TARGET_PATH", &target_dir)
    ).unwrap();

    // run other stuff
    create_dir_all("build/").unwrap();

    let ccargo: CargoToml = {
        let mut f = File::open("Cargo.toml").unwrap();
        let mut contents = String::new();
        f.read_to_string(&mut contents).unwrap();
        from_str(&contents)
            .unwrap()
    };

    Command::new("ld")
            .arg("-n")
            .arg("-T").arg(format!("{}/layout.ld", target_dir))
            .arg("-o").arg(format!("build/metal-{}.bin", target))
            .arg(format!("target/{}-unknown-none/release/lib{}.a", target, &ccargo.package.name))
            .status()
            .unwrap();

    create_dir_all("build/isofiles/boot/grub").unwrap();


    copy(
        format!("build/metal-{}.bin", target), 
        "build/isofiles/boot/metal.bin"
    ).unwrap();

    copy(
        format!("arch/{}/grub.cfg", target),
        "build/isofiles/boot/grub/grub.cfg"
    ).unwrap();

    Command::new("grub-mkrescue")
            .arg("-o").arg(format!("build/{}.iso", target))
            .arg("build/isofiles")
            .status()
            .unwrap();

    remove_dir_all("build/isofiles").unwrap();
}