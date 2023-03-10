// Copyright 2022 Adobe. All rights reserved.
// This file is licensed to you under the Apache License,
// Version 2.0 (http://www.apache.org/licenses/LICENSE-2.0)
// or the MIT license (http://opensource.org/licenses/MIT),
// at your option.
// Unless required by applicable law or agreed to in writing,
// this software is distributed on an "AS IS" BASIS, WITHOUT
// WARRANTIES OR REPRESENTATIONS OF ANY KIND, either express or
// implied. See the LICENSE-MIT and LICENSE-APACHE files for the
// specific language governing permissions and limitations under
// each license.

#![doc = include_str!("../README.md")]
/// Tool to display and create C2PA manifests
///
/// A file path to an asset must be provided
/// If only the path is given, this will generate a summary report of any claims in that file
/// If a manifest definition json file is specified, the claim will be added to any existing claims
///
use std::fs::{create_dir_all, remove_dir_all, File};
use std::io::Write;
use std::path::PathBuf;

use anyhow::{anyhow, bail, Context, Result};
use c2pa::{Error, Ingredient, Manifest, ManifestStore, ManifestStoreReport};
use clap::{AppSettings, Parser};

mod info;
use info::info;
mod signer;
use signer::SignConfig;

/// Tool for displaying and creating C2PA manifests."
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None, setting = AppSettings::ArgRequiredElseHelp)]
struct CliArgs {
    #[clap(short, long, help = "Path to manifest definition JSON file.")]
    manifest: Option<PathBuf>,

    #[clap(short, long, help = "Path to output file or folder.")]
    output: Option<PathBuf>,

    #[clap(short, long, help = "Path to a parent file.")]
    parent: Option<PathBuf>,

    #[clap(short, long, help = "Manifest definition passed as a JSON string.")]
    config: Option<String>,

    #[clap(short, long, help = "Display detailed C2PA-formatted manifest data.")]
    detailed: bool,

    #[clap(short, long, help = "Force overwrite of output if it already exists.")]
    force: bool,

    #[clap(help = "The path to an asset to examine or embed a manifest into.")]
    path: PathBuf,

    #[clap(short, long, help = "Embed remote URL manifest reference.")]
    remote: Option<String>,

    #[clap(short, long, help = "Generate a sidecar (.c2pa) manifest")]
    sidecar: bool,

    #[clap(short, long, help = "Write ingredient report and assets to a folder.")]
    ingredient: bool,

    #[clap(long, help = "Create a tree diagram of the manifest store.")]
    tree: bool,

    #[clap(long = "certs", help = "Extract certificate chain.")]
    cert_chain: bool,

    #[clap(long, help = "Show manifest size, XMP url and other stats")]
    info: bool,
}

// convert certain errors to output messages
fn special_errs(e: c2pa::Error) -> anyhow::Error {
    match e {
        Error::JumbfNotFound => anyhow!("No claim found"),
        Error::FileNotFound(name) => anyhow!("File not found: {}", name),
        Error::UnsupportedType => anyhow!("Unsupported file type"),
        Error::PrereleaseError => anyhow!("Prerelease claim found"),
        _ => e.into(),
    }
}

fn main() -> Result<()> {
    let args = CliArgs::parse();

    // set RUST_LOG=debug to get detailed debug logging
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "error");
    }
    env_logger::init();

    let path = &args.path;

    if args.info {
        return info(path);
    }

    if args.cert_chain {
        ManifestStoreReport::dump_cert_chain(path)?;
        return Ok(());
    }

    if args.tree {
        ManifestStoreReport::dump_tree(path)?;
        return Ok(());
    }

    // Remove manifest needs to also remove XMP provenance
    // if args.remove_manifest {
    //     match args.output {
    //         Some(output) => {
    //             if output.exists() && !args.force {
    //                 bail!("Output already exists, use -f/force to force write");
    //             }
    //             if path != &output {
    //                 std::fs::copy(path, &output)?;
    //             }
    //             Manifest::remove_manifest(&output)?
    //         },
    //         None => {
    //             bail!("The -o/--output argument is required for this operation");
    //         }
    //     }
    //     return Ok(());
    // }

    // get manifest config from either the -manifest option or the -config option
    let (manifest_opt, sign_config) = if let Some(json) = args.config {
        if args.manifest.is_some() {
            bail!("Do not use config and manifest options together");
        }
        (
            Some(Manifest::from_json(&json)?),
            SignConfig::from_json(&json)?,
        )
    } else if let Some(manifest_path) = args.manifest {
        let json = std::fs::read_to_string(&manifest_path)?;
        let mut manifest = Manifest::from_json(&json)?;
        let mut sign_config = SignConfig::from_json(&json)?;
        if let Some(base) = manifest_path.parent() {
            manifest.resources_mut().set_base_path(base);
            sign_config.set_base_path(base);
        }
        (Some(manifest), sign_config)
    } else {
        (
            None,
            SignConfig {
                ..Default::default()
            },
        )
    };

    // if we have a manifest config, process it
    if let Some(mut manifest) = manifest_opt {
        if let Some(parent_path) = args.parent {
            manifest.set_parent(c2pa::Ingredient::from_file(&parent_path)?)?;
        }

        // If the source file has a manifest store, and no parent is specified treat the source as a parent.
        // note: This could be treated as an update manifest eventually since the image is the same
        if manifest.parent().is_none() {
            let source_ingredient = Ingredient::from_file(&args.path)?;
            if source_ingredient.manifest_data().is_some() {
                manifest.set_parent(source_ingredient)?;
            }
        }

        if let Some(remote) = args.remote {
            if args.sidecar {
                manifest.set_embedded_manifest_with_remote_ref(remote);
            } else {
                manifest.set_remote_manifest(remote);
            }
        } else if args.sidecar {
            manifest.set_sidecar_manifest();
        }

        if let Some(output) = args.output {
            if output.extension() != args.path.extension() {
                bail!("output type must match source type");
            }
            if output.exists() && !args.force {
                bail!("Output already exists, use -f/force to force write");
            }

            if output.file_name().is_none() {
                bail!("Missing filename on output");
            }
            if output.extension().is_none() {
                bail!("Missing extension output");
            }

            // create any needed folders for the output path (embed should do this)
            let mut output_dir = PathBuf::from(&output);
            output_dir.pop();
            manifest.resources_mut().set_base_path(&output_dir);
            create_dir_all(&output_dir)?;

            let signer = sign_config.signer()?;

            manifest
                .embed(&args.path, &output, signer.as_ref())
                .context("embedding manifest")?;

            // generate a report on the output file
            if args.detailed {
                println!(
                    "{}",
                    ManifestStoreReport::from_file(&output).map_err(special_errs)?
                );
            } else {
                println!(
                    "{}",
                    ManifestStore::from_file(&output).map_err(special_errs)?
                )
            }
        } else if args.detailed {
            bail!("Detailed report not supported for preview");
        } else {
            // normally the output file provides the title, format and other manifest fields
            // since there is no output file, gather some information from the source
            if let Some(extension) = args
                .path
                .extension()
                .map(|e| e.to_string_lossy().to_string())
            {
                // set the format field
                match extension.as_str() {
                    "jpg" | "jpeg" => {
                        manifest.set_format("image/jpeg");
                    }
                    "png" => {
                        manifest.set_format("image/png");
                    }
                    _ => (),
                }
            }
            println!("{}", ManifestStore::from_manifest(&manifest)?)
        }
    } else if args.parent.is_some() || args.sidecar || args.remote.is_some() {
        bail!("manifest definition required with these options or flags")
    } else if let Some(output) = args.output {
        if output.is_file() || output.extension().is_some() {
            bail!("Output must be a folder for this option.")
        }
        if output.exists() {
            if args.force {
                remove_dir_all(&output)?;
            } else {
                bail!("Output already exists, use -f/force to force write");
            }
        }
        create_dir_all(&output)?;
        if args.ingredient {
            let report = Ingredient::from_file_with_folder(&args.path, &output)
                .map_err(special_errs)?
                .to_string();
            File::create(output.join("ingredient.json"))?.write_all(&report.into_bytes())?;
            println!("Ingredient report written to the directory {:?}", &output);
        } else {
            let report = ManifestStore::from_file_with_resources(&args.path, &output)
                .map_err(special_errs)?
                .to_string();
            if args.detailed {
                // for a detailed report first call the above to generate the thumbnails
                // then call this to add the detailed report
                let detailed = ManifestStoreReport::from_file(&args.path)
                    .map_err(special_errs)?
                    .to_string();
                File::create(output.join("detailed.json"))?.write_all(&detailed.into_bytes())?;
            }
            File::create(output.join("manifest_store.json"))?.write_all(&report.into_bytes())?;
            println!("Manifest report written to the directory {:?}", &output);
        }
    } else if args.detailed {
        println!(
            "{}",
            ManifestStoreReport::from_file(&args.path).map_err(special_errs)?
        )
    } else {
        println!(
            "{}",
            ManifestStore::from_file(&args.path).map_err(special_errs)?
        )
    }

    Ok(())
}

#[cfg(test)]
pub mod tests {
    #![allow(clippy::unwrap_used)]

    use super::*;
    const CONFIG: &str = r#"{
        "alg": "es256",
        "private_key": "es256_private.key",
        "sign_cert": "es256_certs.pem",
        "ta_url": "http://timestamp.digicert.com",
        "assertions": [
            {
                "label": "org.contentauth.test",
                 "data": {"my_key": "whatever I want"}
            }
        ]
    }"#;

    #[test]
    fn test_manifest_config() {
        const SOURCE_PATH: &str = "tests/fixtures/earth_apollo17.jpg";
        const OUTPUT_PATH: &str = "target/tmp/unit_out.jpg";
        create_dir_all("target/tmp").expect("create_dir");
        let mut manifest = Manifest::from_json(CONFIG).expect("from_json");

        let signer = SignConfig::from_json(CONFIG)
            .unwrap()
            .set_base_path("sample")
            .signer()
            .expect("get_signer");

        let _result = manifest
            .embed(SOURCE_PATH, OUTPUT_PATH, signer.as_ref())
            .expect("embed");

        let ms = ManifestStore::from_file(&OUTPUT_PATH)
            .expect("from_file")
            .to_string();
        //let ms = report_from_path(&OUTPUT_PATH, false).expect("report_from_path");
        assert!(ms.contains("my_key"));
    }
}
