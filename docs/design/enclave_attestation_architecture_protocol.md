# Introduction

It mainly introduces the communication protocol between `Verdictd` and it's client(such as [Attestation Agent](https://github.com/confidential-containers/attestation-agent).
This protocol's communication channel should be encrypted and built based on remote attestation (such as [rats-tls](https://github.com/inclavare-containers/rats-tls)).

# Version

It is used to query the protocol's version number.

## Request

```JSON
{
    "command": "version"
}
```

## Response

```JSON
{
    "status": "OK",
    "version": "v1"
}
```

# Echo

This command's response will echo the `request.data` content.

## Request

```JSON
{
    "command": "echo"
    "data": "xxxxxxx"
}
```

## Response

```
xxxxxxx
```

# Decryption

Decrypt the `blobs[x].encrypted_data` with `blobs[x].kid` corresponding key, `blobs[x].iv` and `blobs[x].algorithm`.

## Request

```JSON
{
    "command": "Decrypt",
    "blobs": [
        {"kid": "xxxxx", "encrypted_data": "xxx<base64encode>", "algorithm": "AES", "key_length": 256, "iv", "xxx<base64encode>"},
        {"kid": "xxxxx", "encrypted_data": "xxx<base64encode>", "algorithm": "AES", "key_length": 256, "iv", "xxx<base64encode>"}
    ]
}
```

## Response

It will respond to the decrypted data if it's executed successfully, or it will respond to the error information.

### Success

```JSON
{
    "status": "OK",
    "data": {
    	"encrypted data1<base64encode>": "decrypted data1<base64encode>",
    	"encrypted data2<base64encode>": "decrypted data2<base64encode>",
    }
    "error": null
}
```

### Failed

```JSON
{
    "status": "Fail",
    "data": null
    "error": "Can't find $(kid)'s corresponding KEK / Decryption data with $(kid)'s KEK failed",
}
```

# Get KEK

Fetch `kids`'s kid corresponding keys.

## Request

```JSON
{
    "command": "Get KEK",
    "kids" : [
        "<resource_path1>",
        "<resource_path2>",
    ]
}
```

## Response

It will respond to these keys if it's executed successfully, or it will respond to the error information.

Here a `resource_path` is the `<repo>/<type>/<tag>` field of a KBS Resource URI

### Success

```JSON
{
    "status": "OK"
    "data": {
        "32sdsd": "xxx<base64encode>"
        "ryjhu66": "xxx<base64encode>"
    }
    "error": null
}
```

### Failed

```JSON
{
    "status": "Fail"
    "data": null
    "error": "Can't find $(kid)'s corresponding KEK"
}
```

# Get Resources

Get the `policy.json` file which is relied on by container image signature's verification.

## Request

```JSON
{
    "command": "Get Resource",
    "optional":{
        "resource_path": "<resource_path>"
    } 
}
```

### Response

It will respond the `<resource_path>` file if it's executed successfully. Here a `resource_path` is the `<repo>/<type>/<tag>` field of a KBS Resource URI

### Success

Directly return the base64 encoded policy file.
### Failed

Send base64 encoded error json string:

```json
{
    "status": "Fail",
    "error": ""
}
```

# Get Resource Info

Get the information of the resource which will be requested.

## Request

```JSON
{
    "command": "Get Resource Info",
    "name":""
}
```

The `"name"` should be a `<resource_path>`.

## Response

It will respond to the information map of that resource.

### Success

```JSON
{
    "status": "OK",
    "data": {
        "base64size": "4096"
    }
}
```

### Failed

```JSON
{
    "status": "Fail",
    "error": "Can't Get Resource information"
}
```
