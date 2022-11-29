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

use anyhow::Result;
use c2pa::{Ingredient, Manifest, ManifestStore, ManifestStoreReport};
use std::fs::{create_dir_all, File};
use std::io;
use std::io::Write;
use std::path::Path;

const THUMBNAIL_CLAIM_NAME: &str = "thumbnail_claim";

// Writes the provided `manifest` thumbnail, if present, to the `destination` path.
fn write_manifest_thumbnail(manifest: &Manifest, destination: &Path) -> Result<()> {
    manifest.thumbnail().map(|(format, bytes)| {
        let name = match format {
            "image/jpg" | "image/jpeg" => format!("{}.{}", THUMBNAIL_CLAIM_NAME, "jpeg"),
            "image/png" => format!("{}.{}", THUMBNAIL_CLAIM_NAME, "png"),
            _ => THUMBNAIL_CLAIM_NAME.to_owned(),
        };
        File::create(destination.join(name))?.write_all(bytes)
    });
    Ok(())
}

// Writes each of the provided `ingredient`'s thumbnails, if present, to the `destination` path.
fn write_ingredient_thumbnails(ingredients: &[Ingredient], destination: &Path) -> Result<()> {
    ingredients
        .iter()
        .filter_map(|ingredient| {
            ingredient
                .thumbnail()
                .map(|(_, bytes)| (ingredient.title(), bytes))
        })
        .map(|(title, bytes)| File::create(destination.join(title))?.write_all(bytes))
        .collect::<Result<Vec<()>, io::Error>>()?;
    Ok(())
}

/// Writes the report of the manifest, including the manifest's thumbnails and ingredient thumbnails,
/// to the provided `destination_path`.
pub(crate) fn write_report_for_path(
    manifest_path: &Path,
    destination_path: &Path,
    is_detailed: bool,
) -> Result<()> {
    let store = ManifestStore::from_file(manifest_path)?;
    create_dir_all(destination_path)?;

    store
        .manifests()
        .iter()
        .enumerate()
        .map(|(i, (_, manifest))| {
            let label = manifest
                .label()
                .map(str::to_string)
                .unwrap_or_else(|| format!("manifest_{}", i))
                .replace(['/', ':'], "-");

            let claim_path = destination_path.join(label);
            create_dir_all(&claim_path)?;

            write_manifest_thumbnail(manifest, &claim_path)
                .and_then(|_| write_ingredient_thumbnails(manifest.ingredients(), &claim_path))
        })
        .collect::<Result<Vec<()>>>()?;

    let manifest_bytes = match is_detailed {
        true => ManifestStoreReport::from_file(manifest_path)?.to_string(),
        false => store.to_string(),
    }
    .into_bytes();

    File::create(destination_path.join("manifest.json"))?.write_all(&manifest_bytes)?;
    Ok(())
}
