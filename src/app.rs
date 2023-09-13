use std::{
    collections::HashSet,
    io::{stdout, Write},
    process::{exit, Stdio},
};

use anyhow::anyhow;

use crate::parser::{parse_log, CacheInfo};

fn from_csv(input: &str) -> anyhow::Result<HashSet<String>> {
    Ok(input.split(",").map(|v| v.trim().to_owned()).collect())
}

pub fn run_app() -> anyhow::Result<()> {
    let mut cmd = std::process::Command::new("nix");
    cmd.stderr(Stdio::piped())
        .arg("build")
        .arg("--dry-run")
        .arg("-v")
        .arg("--log-format")
        .arg("raw");

    if let Ok(build_args) = std::env::var("EXTRA_BUILD_ARG") {
        for arg in build_args.split(" ") {
            cmd.arg(arg);
        }
    }

    cmd.arg(std::env::var("DERIVATION")?);

    let output = cmd.output().map_err(|e| {
        anyhow!("Failed to spawn the Nix build: {:?}", e)
    })?;

    if !output.status.success() {
        stdout().write_all(&output.stderr)?;
        exit(1);
    }

    let stderr = output.stderr;

    let cache_info = parse_log(stderr.as_slice())?;

    println!(
        "Found [{}] to derivations build and [{}] to fetch",
        cache_info.get_derivations_to_build().len(),
        cache_info.get_derivations_to_fetch().len()
    );
    if validate(
        from_csv(std::env::var("PERMIT_BUILD_DERIVATIONS").unwrap_or_else(|_| "".to_string()).as_str())?,
        &cache_info,
    ) {
        println!("Validation passed!");
    } else {
        eprintln!("Validation failed!");
        eprintln!(
            "To build: \n\t{}",
            cache_info.get_derivations_to_build().join("\n\t")
        );
        eprintln!(
            "To fetch: \n\t{}",
            cache_info.get_derivations_to_fetch().join("\n\t")
        );
        return Err(anyhow!("Validation failed"));
    }

    Ok(())
}

pub fn validate(permit_build_derivations: HashSet<String>, cache_info: &CacheInfo) -> bool {
    let mut all_passed = true;

    for to_build in cache_info.get_derivations_to_build() {
        let to_build_name: String = to_build
            .split(".")
            .next()
            .unwrap()
            .split("-")
            .skip(1)
            .collect::<Vec<&str>>()
            .join("-");

        if permit_build_derivations.contains(to_build_name.as_str()) {
            println!("Permitting [{}] to be built", to_build);
            continue;
        }

        eprintln!(
            "Found [{}] which requires building but is not in the permitted set",
            to_build
        );
        all_passed = false;
    }

    all_passed
}
