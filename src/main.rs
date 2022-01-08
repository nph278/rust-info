use cargo_toml::{Edition::*, Manifest};
use colored::Colorize;
use std::process::exit;

fn format_bytes(a: u64) -> String {
    if a < 1000 {
        format!("{} bytes", a)
    } else if a < 1000 {
        format!("{} bytes", a)
    } else if a < 1000000 {
        format!("{}KB", a / 1000)
    } else if a < 1000000000 {
        format!("{}MB", a / 1000000)
    } else if a < 1000000000000 {
        format!("{}MB", a / 1000000000)
    } else {
        format!("it fills up your hard drive")
    }
}

fn main() {
    let manifest = match Manifest::from_path("Cargo.toml") {
        Ok(m) => m,
        Err(e) => {
            println!("{}", e.to_string().red());
            exit(1);
        }
    };

    let unknown_string = "[unknown]".to_string();

    let package = manifest.package;
    let name = package
        .as_ref()
        .map(|a| a.name.clone())
        .unwrap_or(unknown_string.clone());
    println!(
        "{}",
        format!(
            "{} v{}:",
            name,
            package
                .as_ref()
                .map(|a| a.version.clone())
                .unwrap_or(unknown_string.clone()),
        )
        .green(),
    );
    println!("");

    println!(
        "{}",
        format!(
            "Rust edition: {}",
            match package.as_ref().map(|a| a.edition.clone()).unwrap_or(E2021) {
                E2015 => "2015",
                E2018 => "2018",
                E2021 => "2021",
            },
        )
        .red(),
    );
    println!("");

    println!(
        "{}",
        format!("dependencies: {}", manifest.dependencies.iter().len()).blue()
    );
    println!(
        "{}",
        format!(
            "dev dependencies: {}",
            manifest.dev_dependencies.iter().len()
        )
        .blue()
    );
    println!(
        "{}",
        format!(
            "build dependencies: {}",
            manifest.build_dependencies.iter().len()
        )
        .blue()
    );
    println!("");

    println!(
        "{}",
        format!(
            "build weight: {}",
            std::fs::metadata(format!("target/debug/{}", name))
                .map(|a| format_bytes(a.len()))
                .unwrap_or(unknown_string)
        )
        .yellow()
    )
}
