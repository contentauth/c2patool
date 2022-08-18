# c2patool - C2PA command line tool

`c2patool` is a command line tool for working with C2PA [manifests](https://c2pa.org/specifications/specifications/1.0/specs/C2PA_Specification.html#_manifests) and media assets (image or video files). 

Use the tool to:

- Read a JSON report of C2PA manifests in [supported file formats](#supported-file-formats).
- Read a low-level report of C2PA manifest data in [supported file formats](#supported-file-formats).
- Preview manifest data from a [manifest definition](#manifest-definition-file).
- Add a C2PA manifest to [supported file formats](#supported-file-formats).

## Installation

PREREQUISITE: Install [Rust](https://www.rust-lang.org/tools/install). 

Enter this command to install or update the tool:

```shell
cargo install c2patool
```

### Updating 

To ensure you have the latest version, enter this command:

```
c2patool -V 
```

The tool will display the version installed.  Compare the version number displayed with the latest release version shown in the [repository releases page](https://github.com/contentauth/c2patool/releases).  To update to the latest version, use the installation command shown above.


## Supported file formats

The tool works with the following types of asset files (also referred to as _assets_).

| MIME type         | extensions  | read only |
| ----------------- | ----------- | --------- |
| `image/jpeg`      | `jpg, jpeg` |           |
| `image/png`       | `png`       |           |
| `image/avif`      | `avif`      |    X      |
| `image/heic`      | `heic`      |    X      |
| `image/heif`      | `heif`      |    X      |
| `video/mp4`       | `mp4`       |           |
| `application/mp4` | `mp4`       |           |
| `audio/mp4`       | `m4a`       |           |
| `application/c2pa`| `c2pa`      |    X      |
| `video/quicktime` | `mov`       |           |

NOTE: Quicktime (`.mov`) format is not yet fully supported.

## Usage

The tool's command-line syntax is:

```
c2patool [OPTIONS] [path]
```

Where  `<path>` is the path to the asset to read, or a JSON configuration file.

The following table describes the command-line options.

| CLI&nbsp;option&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp; | Short version | Argument | Description |
|-----|----|----|----|
| `--config` | `-c` | `<config>` | Specifies a manifest definition as a JSON string. See [Providing a manifest definition on the command line](#providing-a-manifest-definition-on-the-command-line). |
| `--detailed` | `-d` | N/A | Display detailed C2PA-formatted manifest data.  See [Displaying a detailed manifest report](#displaying-a-detailed-manifest-report). |
| `--force` | `-f` | N/A | Force overwriting output file. See [Forced overwrite](#forced-overwrite). |
| `--help` | `-h` | N/A | Display CLI help information. |
| `--output` | `-o` | `<output_file>` | Specifies path to output file. See [Adding a manifest to an asset file](#adding-a-manifest-to-an-asset-file). |
| `--manifest` | `-m` | `<manifest_file>` | Specifies a manifest file to add to an asset file. See [Adding a manifest to an asset file](#adding-a-manifest-to-an-asset-file).
| `--parent` | `-p` | `<parent_file>` |  Specifies path to parent file. See [Specifying a parent file](#specifying-a-parent-file). |
| `--remote` | `-r` | `<manifest_url>` | Specify remote manifest available over HTTP. See [Generating a remote manifest](#generating-a-remote-manifest)|
| `--sidecar` | `-s` | N/A | Put manifest in external "sidecar" file with `.c2pa` extension. See [Generating an external manifest](#generating-an-external-manifest). |
| `--version` | `-V` | N/A | Display version information. |

### Displaying manifest data

To display the manifest associated with an asset file, provide the relative path to the file as the argument; for example:

```shell
c2patool image.jpg
```

The tool displays the manifest JSON to standard output (stdout).

### Displaying a detailed manifest report

To display a detailed report describing the internal C2PA format of manifests contained in the asset, use the `-d` option; for example:

```shell
c2patool -d image.jpg
```

The tool displays the detailed report to standard output (stdout).

### Adding a manifest to an asset file

To add C2PA manifest data to a file, use the `--manifest` (or `-m`) option with a manifest JSON file as the option argument and the path to the asset file to be signed.  Specify the output file as the argument to the `-o` or `--output` option.   For example:

```shell
c2patool image.jpg -m sample/test.json -o signed_image.jpg
```

The tool generates a new manifest using the values given in the file and displays the manifest store to standard output (stdout).

CAUTION: If the output file is the same as the source file, the tool will overwrite the source file. 

If you do not use the  `-o` or `--output` option, then the tool will display the generated manifest but will not save it to a file.

#### Specifying a parent file

A _parent file_ represents the state of the image before any edits were made.  

Specify a parent file as the argument to the `--parent`/`-p` option; for example:

```shell
c2patool image.jpg -m sample/test.json -p parent.jpg -o signed_image.jpg
```

You can also specify a parent file in the manifest definition.

#### Forced overwrite

The tool will return an error if the output file already exists. Use the `--force` /  `-f` option to force overwriting the output file.  For example:

```shell
c2patool image.jpg -m sample/test.json -f -o signed_image.jpg
```

### Previewing a manifest

To display a preview of the generated manifest and ensure you've formatted the manifest definition correctly, provide the path to a manifest file as the argument with no other options or flags; for example:

```shell
c2patool sample/test.json
```

### Generating an external manifest

Use the `--sidecar` (or `-s`) option to put the manifest in an external sidecar file in the same location as the output file. The manifest will have the same output filename but with a ".c2pa" extension. The tool will copy the output file but the original will be untouched. 

```shell
c2patool image.jpg -s -m sample/test.json -o signed_image.jpg
```
### Generating a remote manifest

Use the `--remote` (or `-r`) option to places an HTTP reference to the manifest in the output file. The manifest is returned as an external sidecar file in the same location as the output file with the same filename but with a ".c2pa" extension. Place the manifest at the location specified by the `-r` option. When using remote manifests the remote URL should be publicly accessible to be most useful to users. When verifying an asset, remote manifests are automatically fetched. 

```shell
c2patool image.jpg -r http://my_server/myasset.c2pa -m sample/test.json -o signed_image.jpg
```

In the example above, the tool tries to fetch the manifest for `new_manifest.jpg` from `http://my_server/myasset.c2pa` during validation.

If you use both the `-s` and `-r` options, the tool embeds a manifest in the output files and also adds the remote reference.

### Providing a manifest definition on the command line

To provide the [manifest definition](#manifest-definition-file) in a command line argument instead of a file, use the `--config` (or `-c`) option.

For example, the following command adds a custom assertion called "org.contentauth.test".

```shell
c2patool -c '{"assertions": [{"label": "org.contentauth.test", "data": {"my_key": "whatever I want"}}]}'
```

## Manifest definition file 

The manifest definition file is a JSON formatted file with a `.json` extension. 
Relative file paths are interpreted as relative to the location of the definition file unless you specify a `base_path` field.

### Schema 

The schema for the manifest definition file is shown below.

```json
{
	"$schema": "http://json-schema.org/draft-07/schema",
	"$id": "http://ns.adobe.com/c2patool/claim-definition/v1",
	"type": "object",
	"description": "Definition format for claim created with c2patool",
	"examples": [
		{
            "alg": "es256",
            "private_key": "es256_private.key",
            "sign_cert": "es256_certs.pem",
            "ta_url": "http://timestamp.digicert.com",
            "vendor": "myvendor",
            "claim_generator": "MyApp/0.1",
            "parent": "image.jpg",  
            "ingredients": [],
            "assertions": [
				{
					"label": "my.assertion",
					"data": {
						"any_tag": "whatever I want"
					}
				}
			],
		}
    ],
	"required": [
		"assertions",
	],
	"properties": {
		"vendor": {
			"type": "string",
			"description": "Typically an Internet domain name (without the TLD) for the vendor (i.e. `adobe`, `nytimes`). If provided this will be used as a prefix on generated manifest labels."
		},
		"claim_generator": {
			"type": "string",
			"description": "A UserAgent string that will let a user know what software/hardware/system produced this Manifest - names should not contain spaces (defaults to c2patool)."
		},
		"title": {
			"type": "string",
			"description": "A human-readable string to be displayed as the title for this Manifest (defaults to the name of the file this manifest was embedded in)."
		},
		"credentials": {
			"type": "object",
			"description": "An array of W3C verifiable credentials objects defined in the c2pa assertion specification. Section 7."
		},
		"parent": {
			"type": "string",
			"format": "Local file system path",
			"description": "A file path to the state of the asset prior to any changes declared in the manifest definition."
		},
        "Ingredients": {
			"type": "array of string",
			"format": "Array of local file system paths",
			"description": "File paths to assets that were used to modify the asset referenced by this Manifest (if any)."
		},
		"assertions": {
			"type": "object",
			"description": "Objects with label, and data - standard c2pa labels must match values as defined in the c2pa assertion specification."
		},
		"alg": {
			"type": "string",
			"format": "Local file system path",
			"description": "Signing algorithm: one of [ ps256 | ps384 | ps512 | es256 | es384 | es512 | ed25519]. Defaults to es256."
		},
		"ta_url": {
			"type": "string",
			"format": "http URL",
			"description": "A URL to an RFC3161 compliant Time Stamp Authority. If missing there will no secure timestamp."
		},
		"private_key": {
			"type": "string",
			"format": "Local file system path",
			"description": "File path to a private key file."
		},
		"sign_cert": {
			"type": "string",
			"format": "Local file system path",
			"description": "File path to signing cert file."
		},
		"base_path": {
			"type": "string",
			"format": "Local file system path",
			"description": "File path to a folder to use as the base for relative paths in this file."
		},
	},
	"additionalProperties": false
}
```

#### Example manifest definition file

Here's an example of a manifest definition that inserts a CreativeWork author assertion. Copy this JSON into a file to use as a test manifest.

```json
{
    "ta_url": "http://timestamp.digicert.com",
    "claim_generator": "TestApp",
    "assertions": [
        {
            "label": "stds.schema-org.CreativeWork",
            "data": {
                "@context": "https://schema.org",
                "@type": "CreativeWork",
                "author": [
                    {
                        "@type": "Person",
                        "name": "Joe Bloggs"
                    }
                ]
            }
        }
    ]
}
```
