use std::{
    collections::HashSet,
    process::Stdio,
};

use clap::Parser;
use crate::parser::{parse_log, CacheInfo};

#[derive(Parser)]
pub struct App {
    #[arg(long)]
    derivation: String,

    #[arg(long)]
    extra_build_args: Option<String>,

    #[arg(long, value_parser = from_csv)]
    permit_build_derivations: Option<HashSet<String>>,
}

fn from_csv(input: &str) -> anyhow::Result<HashSet<String>> {
    Ok(input.split(",").map(|v| v.trim().to_owned()).collect())
}

pub fn run_app(args: App) -> anyhow::Result<()> {
    let mut cmd = std::process::Command::new("nix");
    cmd.stderr(Stdio::piped())
        .arg("build")
        .arg("--dry-run")
        .arg("-v")
        .arg("--log-format")
        .arg("raw");

    if let Some(extra_build_arg) = &args.extra_build_args {
        cmd.arg(extra_build_arg);
    }

    cmd.arg(args.derivation.as_str());

    let stderr = cmd.output()?.stderr;

    let cache_info = parse_log(stderr.as_slice())?;

    println!("Found [{}] to derivations build and [{}] to fetch", cache_info.get_derivations_to_build().len(), cache_info.get_derivations_to_fetch().len());
    if validate(&args, &cache_info) {
        println!("Validation passed!");
    } else {
        eprintln!("Validation failed!");
        eprintln!("To build: \n\t{}", cache_info.get_derivations_to_build().join("\n\t"));
        eprintln!("To fetch: \n\t{}", cache_info.get_derivations_to_fetch().join("\n\t"));
    }

    Ok(())
}

pub fn validate(args: &App, cache_info: &CacheInfo) -> bool {
    let permit_build_derivations = args
        .permit_build_derivations
        .clone()
        .unwrap_or_else(|| HashSet::new());

    let mut all_passed = true;

    for to_build in cache_info.get_derivations_to_build() {
        let to_build_name: String = to_build.split(".").next().unwrap().split("-").skip(1).collect::<Vec<&str>>().join("-");

        if permit_build_derivations.contains(to_build_name.as_str()) {
            println!("Permitting [{}] to be built", to_build);
            continue;
        }

        eprintln!("Found [{}] which requires building but is not in the permitted set", to_build);
        all_passed = false;
    }

    all_passed
}