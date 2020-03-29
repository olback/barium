# Barium Server

#### Multithreaded Chat Server

Example configuration:

```json
{
    "cert": {
        "path": "server/cert/cert.p12",
        "password": "1234"
    },
    "server": {
        "address": "0.0.0.0",
        "port": 13337,
        "password": null
    },
    "runtime": {
        "core_threads": null,
        "max_threads": null
    },
    "blacklist": [
        "10.8.0.0/24",
        "187.63.59.237"
    ],
    "log_level": "info"
}
```


### Cert

**Path:** *`string`* Path to certificate in `PKCS #12` format. Usually a file ending in `pfx` or `p12`.
**Password:** *`string`* The password used to encrypt the certificate file.


### Server

**Address:** *`string`* What address to listen on. Should be the same as the domain the certificate is valid for.
**Port:** *`number`* What port to listen on. Default is `13337`.
**Password:** *`string | null`* Password used to used for authentication. Set to `null` to disable password authentication.


### Runtime

**Core threads:** *`number | null`* Maximum amount of physical cores used by the tokio runtime. Set to `null` to use all available cores.
**Max threads:** *`number | null`* Maximum amount of green threads to spawn. This amount determines how many clients can connect. Set to `null` to use the default. Read more [here](https://docs.rs/tokio/0.2.13/tokio/runtime/struct.Builder.html#method.max_threads).


**Blacklist:** *`[string]`* This is an array where ip addresses and ip ranges can be blocked. Leave empty to not block anything. Example:
```json
...
"blacklist": [
    "1.2.3.0/24",
    "2.3.4.5",
    "3.0.0.0/8"
]
...
```


**Log level:** *`"off", "error", "warn", "info", "debug", "trace" | null`* Sets the log level. Setting this to `null` will use the default value which is `info` for production builds and `trace` for debug builds.

