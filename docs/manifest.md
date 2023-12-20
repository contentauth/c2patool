# Manifest definition file

The c2patool reads a manifest definition JSON file with a `.json` extension.  This file defines a single manifest to be added to an asset's manifest store.
In the manifest definition file, file paths are relative to the location of the file unless you specify a `base_path` field.

## Example

The example below is a snippet of a manifest definition that inserts a CreativeWork author assertion. This example uses the default testing certificates in the [sample folder](https://github.com/contentauth/c2patool/tree/main/sample) that are also built into the c2patool binary.   Copy this JSON into a file to use as a test manifest. 

**NOTE**:  When you don't specify a key or certificate in the manifest `private_key` and `sign_cert` fields, the tool will use the built-in key and cert. You'll see a warning message, since they are meant for development purposes only. For actual use, provide a permanent key and certificate in the manifest definition or environment variables; see [Creating and using an X.509 certificate](x_509.md). 

The following manifest properties are specific to c2patool and used for signing manifests:

- `alg`: Signing algorithm to use. See [Creating and using an X.509 certificate](x_509.md) for possible values. Default: `es256`.
- `private_key`: Private key to use. Default: `es256_private.key`
- `sign_cert`: Signing certificate to use. Default: `es256_certs.pem`
- `ta_url`:  Time Authority URL for getting a time-stamp (for example, `http://timestamp.digicert.com`). A time-stamp provides a way to confirm that the manifest was signed when the certificate was valid, even if the certificate has since expired. Howver, the Time Authority URL requires a live online connection for confirmation, which may not always be available.

```json
{
    "alg": "es256",
    "private_key": "es256_private.key",
    "sign_cert": "es256_certs.pem",
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

## JSON schemas

* [Manifest store schema](https://opensource.contentauthenticity.org/docs/manifest/manifest-json-schema)

* [Manifest definition schema](https://github.com/contentauth/c2patool/blob/main/schemas/manifest-definition.json)

* [Ingredient schema](https://github.com/contentauth/c2patool/blob/main/schemas/ingredient.json)
