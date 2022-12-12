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
use std::path::{Path, PathBuf};

const THUMBNAIL_CLAIM_NAME: &str = "thumbnail_claim";

fn add_extension(file_name: &str, media_type: &str) -> String {
    let media_type_extension = match media_type {
        "image/jpg" | "image/jpeg" => Some("jpg"),
        "image/png" => Some("png"),
        _ => None,
    };

    media_type_extension
        .and_then(|extension| {
            let mut path = PathBuf::from(&file_name);
            path.set_extension(extension);
            path.into_os_string().into_string().ok()
        })
        .unwrap_or_else(|| file_name.to_owned())
}

// Writes the provided `manifest` thumbnail, if present, to the `destination` path.
fn write_manifest_thumbnail(manifest: &Manifest, destination: &Path) -> Result<()> {
    manifest.thumbnail().map(|(format, bytes)| {
        let name = add_extension(THUMBNAIL_CLAIM_NAME, format);
        File::create(destination.join(name))?.write_all(bytes)
    });
    Ok(())
}

// Writes each of the provided `ingredient`'s thumbnails, if present, to the `destination` path.
fn write_ingredient_thumbnails(ingredients: &[Ingredient], destination: &Path) -> Result<()> {
    ingredients
        .iter()
        .filter_map(|i| {
            i.thumbnail().map(|(format, bytes)| {
                let title = add_extension(i.title(), format);
                (title, bytes)
            })
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

    write_html_report(manifest_path, destination_path)?;
    Ok(())
}

// adds an html tree structured view to the report
pub(crate) fn write_html_report(manifest_path: &Path, destination_path: &Path) -> Result<()> {
    let html_start = r#"<!DOCTYPE html>
    <html>
        <head>
        <meta charset="utf-8">
        <style>
            img {
                height: 75px;
            } 
        </style>
        </head>
        <body> 
    "#;
    let html_end = r#"
        </body>
    </html>
    "#;

    fn add_manifest(
        manifest_store: &ManifestStore,
        manifest_label: &str,
        mut html: String,
    ) -> Result<String> {
        if let Some(manifest) = manifest_store.get(manifest_label) {
            let manifest_label = manifest_label.replace(['/', ':'], "-");
            let claim_img = format!(
                "<img src=\"{}/thumbnail_claim.jpg\"></img>\n{}<p>\n",
                manifest_label, manifest.title().unwrap_or("Untitled")
            );
            let mut content = claim_img;
            if !manifest.ingredients().is_empty() {
                content += "<ul class=tree>\n";
                for ingredient in manifest.ingredients().iter() {
                    let img = format!(
                        "<li><img src=\"{}/{}\"></img>{}\n",
                        &manifest_label,
                        &ingredient.title(),
                        &ingredient.title()
                    );
                    content += &img;
                    if let Some(label) = ingredient.active_manifest() {
                        html = add_manifest(manifest_store, label, html)?;
                    };
                }
                html += &(content + "</ul>\n");
            }
        }
        Ok(html)
    }

    let manifest_store = ManifestStore::from_file(manifest_path)?;
    create_dir_all(destination_path)?;

    let mut content = html_start.to_string();
    content += &format!("<h3>Report for: {:?}</h3>\n", &manifest_path);
    content += r#"<ul class="tree">"#;
    content += "\n";
    if let Some(manifest_label) = manifest_store.active_label() {
        content = add_manifest(&manifest_store, manifest_label, content)?;
    }
    content += "</ul>\n";
    content += html_end;

    File::create(destination_path.join("manifest.html"))?.write_all(content.as_bytes())?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::fs_report::add_extension;

    #[test]
    fn test_file_extension() {
        assert_eq!(add_extension("filename.png", "image/png"), "filename.png");
    }

    #[test]
    fn test_file_extension_not_equal_ingredient_name() {
        assert_eq!(add_extension("filename.psd", "image/png"), "filename.png");
    }

    #[test]
    fn test_file_extension_ingredient_has_no_extension() {
        assert_eq!(add_extension("filename", "image/jpeg"), "filename.jpg");
    }

    #[test]
    fn test_file_extension_ingredient_filename_has_period_in_name() {
        assert_eq!(
            add_extension("filename.final.version", "image/jpeg"),
            "filename.final.jpg"
        );
    }
}
