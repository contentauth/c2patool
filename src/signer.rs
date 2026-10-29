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
    collections::BTreeMap,
    env,
    path::{Path, PathBuf},
};

use anyhow::{bail, Context, Result};
use c2pa::{create_signer, dynamic_assertion::DynamicAssertion, BoxedSigner, Signer, SigningAlg};
use serde::Deserialize;

const DEFAULT_CERTS: &[u8] = include_bytes!("../sample/es256_certs.pem");
const DEFAULT_KEY: &[u8] = include_bytes!("../sample/es256_private.key");

pub fn get_ta_url() -> Option<String> {
    std::env::var("C2PA_TA_URL").ok()
}

/// Time Authority request headers from `C2PA_TA_HEADERS`, one `Name: value` per line.
pub fn get_ta_headers() -> Result<Vec<(String, String)>> {
    let Ok(lines) = env::var("C2PA_TA_HEADERS") else {
        return Ok(Vec::new());
    };

    lines
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .map(|line| {
            // Don't echo the line in the error, it's likely a secret.
            let (name, value) = line
                .split_once(':')
                .context("C2PA_TA_HEADERS: each line must be `Name: value`")?;
            Ok((name.trim().to_owned(), value.trim().to_owned()))
        })
        .collect()
}

fn validate_ta_header_name(name: &str) -> Result<()> {
    if name.is_empty() || name.contains(char::is_whitespace) {
        bail!("Invalid Time Authority header name: {name:?}");
    }
    if name.eq_ignore_ascii_case("content-type") {
        bail!("The Time Authority `Content-Type` header is set by c2pa and can't be overridden");
    }
    Ok(())
}

/// Wraps `signer` so its Time Authority requests also send `headers`.
pub fn with_ta_headers(signer: BoxedSigner, headers: Vec<(String, String)>) -> BoxedSigner {
    if headers.is_empty() {
        return signer;
    }
    Box::new(TaHeadersSigner { signer, headers })
}

/// Adds headers to a signer's Time Authority requests.
///
/// Forwards everything to the wrapped signer except `send_timestamp_request`.
/// The trait default builds the request from this wrapper's
/// `timestamp_request_headers`, and forwarding would drop the added headers.
struct TaHeadersSigner {
    signer: BoxedSigner,
    headers: Vec<(String, String)>,
}

impl Signer for TaHeadersSigner {
    fn sign(&self, data: &[u8]) -> c2pa::Result<Vec<u8>> {
        self.signer.sign(data)
    }

    fn alg(&self) -> SigningAlg {
        self.signer.alg()
    }

    fn certs(&self) -> c2pa::Result<Vec<Vec<u8>>> {
        self.signer.certs()
    }

    fn reserve_size(&self) -> usize {
        self.signer.reserve_size()
    }

    fn time_authority_url(&self) -> Option<String> {
        self.signer.time_authority_url()
    }

    fn timestamp_request_headers(&self) -> Option<Vec<(String, String)>> {
        let mut headers = self.signer.timestamp_request_headers().unwrap_or_default();
        headers.extend(self.headers.iter().cloned());
        Some(headers)
    }

    fn timestamp_request_body(&self, message: &[u8]) -> c2pa::Result<Vec<u8>> {
        self.signer.timestamp_request_body(message)
    }

    fn ocsp_val(&self) -> Option<Vec<u8>> {
        self.signer.ocsp_val()
    }

    fn ocsp_vals(&self) -> Vec<Vec<u8>> {
        self.signer.ocsp_vals()
    }

    fn direct_cose_handling(&self) -> bool {
        self.signer.direct_cose_handling()
    }

    fn dynamic_assertions(&self) -> Vec<Box<dyn DynamicAssertion>> {
        self.signer.dynamic_assertions()
    }
}

/// Output signer info JSON for the `sign-mode --signer-info` subcommand.
///
/// Uses cert/alg from `[signer.local]` settings if configured, otherwise the
/// baked-in es256 test cert.
pub fn output_signer_info(settings_path: &std::path::Path) -> anyhow::Result<()> {
    use std::io::Write;

    // Try to load cert/alg from settings; fall back to DEFAULT_CERTS.
    let (cert_pem, alg, tsa_url) =
        if let Ok(settings) = c2pa::settings::Settings::new().with_file(settings_path) {
            match settings.signer {
                Some(c2pa::settings::signer::SignerSettings::Local {
                    alg,
                    sign_cert,
                    tsa_url,
                    ..
                }) => (sign_cert, alg, tsa_url),
                _ => (
                    String::from_utf8_lossy(DEFAULT_CERTS).into_owned(),
                    SigningAlg::Es256,
                    None,
                ),
            }
        } else {
            (
                String::from_utf8_lossy(DEFAULT_CERTS).into_owned(),
                SigningAlg::Es256,
                None,
            )
        };

    let info = crate::SignerInfo {
        alg,
        sign_cert: cert_pem,
        tsa_url,
        reserve_size: None,
    };
    let json = serde_json::to_string(&info).context("serializing signer info")?;
    std::io::stdout()
        .write_all(json.as_bytes())
        .context("writing signer info to stdout")?;
    Ok(())
}

/// Read raw bytes from stdin, sign them with the baked-in es256 test key, and write
/// the raw signature bytes to stdout.  Used by the `sign-mode` subcommand.
pub fn sign_from_stdin() -> anyhow::Result<()> {
    use std::io::{Read, Write};

    let mut data = Vec::new();
    std::io::stdin()
        .read_to_end(&mut data)
        .context("reading bytes from stdin")?;

    let signer = create_signer::from_keys(DEFAULT_CERTS, DEFAULT_KEY, SigningAlg::Es256, None)
        .context("creating test signer")?;

    let sig = c2pa::Signer::sign(signer.as_ref(), &data).context("signing")?;

    std::io::stdout()
        .write_all(&sig)
        .context("writing signature to stdout")?;

    Ok(())
}
#[derive(Debug, Default, Deserialize)]
pub struct SignConfig {
    /// Signing algorithm to use - must match the associated certs
    ///
    /// Must be one of [ ps256 | ps384 | ps512 | es256 | es384 | es512 | ed25519 ]
    /// Defaults to es256
    pub alg: Option<String>,
    /// A path to a file containing the private key required for signing
    pub private_key: Option<PathBuf>,
    /// A path to a file containing the signing cert required for signing
    pub sign_cert: Option<PathBuf>,
    /// A Url to a Time Authority to use when signing the manifest
    pub ta_url: Option<String>,
    /// HTTP headers to send with Time Authority requests
    pub ta_headers: Option<BTreeMap<String, String>>,
}

impl SignConfig {
    pub fn from_json(json: &str) -> Result<Self> {
        serde_json::from_str(json).context("reading manifest configuration")
    }

    // set a base for all non-absolute paths
    pub fn set_base_path<P: AsRef<Path>>(&mut self, base: P) -> &Self {
        if let Some(path) = self.private_key.as_ref() {
            if !path.is_absolute() {
                self.private_key = Some(base.as_ref().join(path));
            }
        }
        if let Some(path) = self.sign_cert.as_ref() {
            if !path.is_absolute() {
                self.sign_cert = Some(base.as_ref().join(path));
            }
        }
        self
    }

    /// Time Authority request headers from `ta_headers`, then `C2PA_TA_HEADERS`.
    pub fn ta_headers(&self) -> Result<Vec<(String, String)>> {
        let mut headers: Vec<(String, String)> = self
            .ta_headers
            .iter()
            .flatten()
            .map(|(name, value)| (name.clone(), value.clone()))
            .collect();
        headers.extend(get_ta_headers()?);

        for (name, _) in &headers {
            validate_ta_header_name(name)?;
        }
        Ok(headers)
    }

    pub fn signer(&self) -> Result<BoxedSigner> {
        let alg = self.alg.as_deref().unwrap_or("es256").to_lowercase();
        let alg: SigningAlg = alg.parse().map_err(|_| c2pa::Error::UnsupportedType)?;
        let tsa_url = self.ta_url.clone().or_else(get_ta_url);

        let mut private_key = None;
        let mut sign_cert = None;

        if let Some(path) = self.private_key.as_deref() {
            private_key =
                Some(std::fs::read(path).context(format!("Reading private key: {:?}", path))?);
        }

        if private_key.is_none() {
            if let Ok(key) = env::var("C2PA_PRIVATE_KEY") {
                private_key = Some(key.as_bytes().to_vec());
            }
        };

        if let Some(path) = self.sign_cert.as_deref() {
            sign_cert =
                Some(std::fs::read(path).context(format!("Reading sign cert: {:?}", path))?);
        }

        if sign_cert.is_none() {
            if let Ok(cert) = env::var("C2PA_SIGN_CERT") {
                sign_cert = Some(cert.as_bytes().to_vec());
            }
        };

        if let Some(private_key) = private_key {
            if let Some(sign_cert) = sign_cert {
                let signer = create_signer::from_keys(&sign_cert, &private_key, alg, tsa_url)
                    .context("Invalid certification data")?;
                return Ok(signer);
            }
        }

        eprintln!(
            "\n\n-----------\n\n\
            Note: Using default private key and signing certificate. This is only valid for development.\n\
            A permanent key and cert should be provided in the manifest definition or in the environment variables.\n");

        let signer = create_signer::from_keys(DEFAULT_CERTS, DEFAULT_KEY, alg, tsa_url)
            .context("Invalid certification data")?;

        Ok(signer)
    }
}

#[cfg(test)]
pub mod tests {
    #![allow(clippy::unwrap_used)]

    use super::*;
    const CONFIG: &str = r#"{
        "alg": "es256",
        "private_key": "es256_private.key",
        "sign_cert": "es256_certs.pem",
        "ta_url": "http://timestamp.digicert.com"
    }"#;

    #[test]
    fn test_sign_config() {
        let mut sign_config = SignConfig::from_json(CONFIG).expect("from_json");
        sign_config.set_base_path("sample");

        let signer = sign_config.signer().expect("get signer");
        assert_eq!(signer.alg(), SigningAlg::Es256);
    }

    #[test]
    fn test_sign_default() {
        let sign_config = SignConfig::default();

        let signer = sign_config.signer().expect("get signer");
        assert_eq!(signer.alg(), SigningAlg::Es256);
    }

    #[test]
    fn test_sign_from() {
        let sign_config = SignConfig::default();

        let signer = sign_config.signer().expect("get signer");
        assert_eq!(signer.alg(), SigningAlg::Es256);
    }
}
