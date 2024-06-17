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

use std::{
    fs::{self, File},
    path::{Path, PathBuf},
};

use anyhow::{bail, Context, Result};
use c2pa::{jumbf_io, Ingredient, Reader};
use clap::Parser;
use log::error;

use crate::commands::{load_trust_settings, Trust};

#[derive(Debug, Parser)]
pub enum Extract {
    /// Extract the .json or .c2pa manifest.
    Manifest {
        /// Input path to asset.
        path: PathBuf,

        /// Path to output file.
        #[clap(short, long)]
        output: PathBuf,

        /// Extract binary .c2pa manifest.
        #[clap(short, long)]
        binary: bool,

        /// Force overwrite output if it already exists.
        #[clap(short, long)]
        force: bool,

        #[clap(flatten)]
        trust: Trust,
    },
    /// Extract the .json ingredient.
    Ingredient {
        /// Input path to asset.
        path: PathBuf,

        /// Path to output ingredient .json.
        #[clap(short, long)]
        output: PathBuf,

        /// Force overwrite output if it already exists.
        #[clap(short, long)]
        force: bool,

        #[clap(flatten)]
        trust: Trust,
    },
    /// Extract known resources from a manifest (e.g. thumbnails).
    Resources(Resources),
}

#[derive(Debug, Parser)]
pub struct Resources {
    /// Input path(s) to asset(s).
    paths: Vec<PathBuf>,

    /// Path to output folder.
    #[clap(short, long)]
    output: PathBuf,

    /// Force overwrite output and clear children if it already exists.
    #[clap(short, long)]
    force: bool,

    /// Also extract resources that are unknown into binary files (unlike known resources, such as thumbnails).
    #[clap(short, long)]
    unknown: bool,

    #[clap(flatten)]
    trust: Trust,
}

impl Extract {
    pub fn execute(&self) -> Result<()> {
        match self {
            Extract::Manifest {
                path,
                output,
                binary,
                force,
                trust,
            } => {
                if !path.exists() {
                    bail!("Input path does not exist")
                } else if !path.is_file() {
                    bail!("Input path must be a file")
                }

                if output.exists() {
                    if !output.is_file() {
                        bail!("Output path must be a file");
                    } else if !force {
                        bail!("Output path already exists use `--force` to overwrite");
                    }
                }

                load_trust_settings(trust)?;

                match binary {
                    true => {
                        let manifest = jumbf_io::load_jumbf_from_file(path)?;
                        // Validates the jumbf refers to a valid manifest.
                        match c2pa::format_from_path(path) {
                            Some(format) => {
                                Reader::from_manifest_data_and_stream(
                                    &manifest,
                                    &format,
                                    &File::open(path)?,
                                )?;
                            }
                            None => bail!("Path `{}` is missing file extension", path.display()),
                        }
                        fs::write(output, manifest)?;
                    }
                    false => {
                        let reader = Reader::from_file(path)?;
                        fs::write(output, reader.to_string())?;
                    }
                }
            }
            Extract::Ingredient {
                path,
                output,
                force,
                trust,
            } => {
                if !path.exists() {
                    bail!("Input path does not exist")
                } else if !path.is_file() {
                    bail!("Input path must be a file")
                }

                if output.exists() {
                    if !output.is_file() {
                        bail!("Output path must be a file");
                    } else if !force {
                        bail!("Output path already exists use `--force` to overwrite");
                    }
                }

                load_trust_settings(trust)?;

                let ingredient = Ingredient::from_file(path)?;
                fs::write(output, ingredient.to_string())?;
            }
            Extract::Resources(resources) => resources.execute()?,
        }
        Ok(())
    }
}

impl Resources {
    pub fn execute(&self) -> Result<()> {
        if self.paths.is_empty() {
            bail!("Input path does not exist")
        }

        if !self.output.exists() {
            fs::create_dir_all(&self.output)?;
        } else if !self.output.is_dir() {
            bail!("Output path must be a folder");
        } else if !self.force {
            bail!("Output path already exists use `--force` to overwrite and clear children");
        }

        load_trust_settings(&self.trust)?;

        let mut errs = Vec::new();
        for path in &self.paths {
            if path.is_dir() {
                bail!("Input path cannot be a folder when extracting resources");
            }

            if let Err(err) = self.extract_resources(path) {
                error!(
                    "Failed to extract resources from asset at path `{}`, {}",
                    path.display(),
                    err.to_string()
                );
                errs.push(err);
            }
        }

        if !errs.is_empty() {
            bail!(
                "Failed to extract resources from {}/{} assets",
                errs.len(),
                self.paths.len()
            );
        }

        Ok(())
    }

    fn extract_resources(&self, path: &Path) -> Result<()> {
        let reader = Reader::from_file(path)?;
        for manifest in reader.iter_manifests() {
            let manifest_path = self.output.join(
                manifest
                    .label()
                    .context("Failed to get maniest label")?
                    .replace(':', "_"),
            );
            for resource_ref in manifest.iter_resources() {
                if !self.unknown || resource_ref.format != "application/octet-stream" {
                    // TODO: need a method in c2pa-rs to normalize the identifier (removing jumbf tag)
                    //       maybe it should be contained within a struct/enum that impls Display
                    let resource_path = manifest_path.join(&resource_ref.identifier);
                    fs::create_dir_all(
                        resource_path
                            .parent()
                            .context("Failed to find resource parent path from label")?,
                    )?;
                    reader.resource_to_stream(
                        &resource_ref.identifier,
                        File::create(&resource_path)?,
                    )?;
                }
            }
        }

        Ok(())
    }
}
