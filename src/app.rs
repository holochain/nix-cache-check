use std::{
    collections::HashSet,
    io::{stdout, Write},
    process::{exit, Stdio},
};

use anyhow::anyhow;

use crate::parser::{parse_log, CacheInfo};

fn from_csv(input: &str) -> anyhow::Result<HashSet<String>> {
    Ok(input
        .split(",")
        .filter_map(|v| {
            let x = v.trim().to_owned();
            if !x.is_empty() {
                Some(x)
            } else {
                None
            }
        })
        .collect())
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
            if !arg.trim().is_empty() {
                cmd.arg(arg);
            }
        }
    }

    cmd.arg(std::env::var("DERIVATION")?);

    println!("Starting Nix build");
    stdout().flush()?;

    let output = cmd
        .output()
        .map_err(|e| anyhow!("Failed to spawn the Nix build: {:?}", e))?;

    println!("Finished Nix build");
    stdout().flush()?;

    if !output.status.success() {
        println!("Nix build command failed, dumping its logs");
        stdout().write_all(&output.stderr)?;
        exit(1);
    }

    let stderr = String::from_utf8(output.stderr)?;

    let cache_info = match parse_log(stderr.as_str()) {
        Ok(ci) => ci,
        Err(e) => {
            eprintln!("There was a problem parsing the input: {}", e);
            eprintln!("The original input was: {}", stderr);
            exit(1);
        }
    };

    println!(
        "Found [{}] to derivations build and [{}] to fetch",
        cache_info.get_derivations_to_build().len(),
        cache_info.get_derivations_to_fetch().len()
    );
    stdout().flush()?;

    if validate(
        from_csv(
            std::env::var("PERMIT_BUILD_DERIVATIONS")
                .unwrap_or_else(|_| "".to_string())
                .as_str(),
        )?,
        &cache_info,
    )? {
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

pub fn validate(
    permit_build_derivations: HashSet<String>,
    cache_info: &CacheInfo,
) -> anyhow::Result<bool> {
    let mut all_passed = true;

    let mut permits_used = HashSet::new();

    for to_build in cache_info.get_derivations_to_build() {
        let to_build_name: String = to_build
            .strip_suffix(".drv")
            .ok_or(anyhow!("Not a derivation? {}", to_build))?
            .split("-")
            .skip(1)
            .collect::<Vec<&str>>()
            .join("-");

        if permit_build_derivations.contains(to_build_name.as_str()) {
            println!("Permitting [{}] to be built", to_build);
            permits_used.insert(to_build_name);
            continue;
        }

        eprintln!(
            "Found [{}] which requires building but is not in the permitted set",
            to_build
        );
        all_passed = false;
    }

    for unused in permit_build_derivations.difference(&permits_used) {
        println!("Warning: You have marked {} as permitted to be built but it is either cached or no longer part of this derivation", unused);
    }

    Ok(all_passed)
}
