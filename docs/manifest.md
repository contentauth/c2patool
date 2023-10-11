# Manifest store file

The manifest store file is a file in JSON format file with `.json` extension. Relative file paths are interpreted as relative to the location of the definition file unless you specify a `base_path` field.

## Example manifest definition file

The c2patool repository contains default certificates in the [sample folder](https://github.com/contentauth/c2patool/tree/main/sample) that are also built into the c2patool binary. 

The example below is a snippet of a manifest definition that inserts a CreativeWork author assertion. This example uses the default testing certificate.  

Copy this JSON into a file to use as a test manifest. You will see a warning message when using them, since they are meant for development purposes only.

**NOTE**: Use the default private key and signing certificate only for development. For actual use, provide a permanent key and certificate in the manifest definition or environment variables; see [Creating and using an X.509 certificate](x_509.md).

It is important to provide a value for the Time Authority URL (the `ta_url` property) to have a valid timestamp on the claim.  NOTE: Only c2patool supports `ta_url` property; it's not part of the C2PA spec or the CAI SDKs.

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

## JSON schemas

* [Schema for the Manifest Definition](https://github.com/contentauth/c2patool/blob/main/schemas/manifest-definition.json)

* [Schema for Ingredient](https://github.com/contentauth/c2patool/blob/main/schemas/ingredient.json)
