# `aius-authd` – Authentication service plugged on LDAP server

## Routes

### `POST /token`

  - Logs a user in, and returns a token
  - Payload:

    ```json
    {
      "username": "<user>",
      "password": "<pass>"
    }
    ```

  - Response:

    ```json
    {
      "token": "<UUID>"
    }
    ```

### `GET /token/:uuid`

  - Get informations about a token
  - Response:

    ```json
    {
      "username": "<user>",
      "scopes": ["checkout", ...]
    }
    ```

### `DELETE /token/:uuid`

  - Revokes a token

## Configuration

### CLI usage

```
USAGE:
    aius-authd [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -c, --config <FILE>                  Path to the config file
        --ldap-base-dn <BASE_DN>         Base DN to bind when connecting to the LDAP server
        --ldap-pass <PASSWORD>           Password used to authenticate with the LDAP server
        --ldap-uri <URI>                 URI used to connect to the LDAP server
        --ldap-user <USER>               Username used to authenticate with the LDAP server
        --ldap-user-pattern <PATTERN>    Pattern used to search users. Ex: `CN=%USER%,OU=people,DC=example,DC=org`
        --redis-uri <URI>                URI used to connect to the redis database
    -a, --server-address <ADDRESS>       Address used for the web server
    -p, --server-port <PORT>             Port used for the web server
```

All options can be either specified in a TOML config file, or via environment variables.

### TOML config file

```toml
[server]
address = "127.0.0.1"
port = "8000"

[ldap]
uri = "ldap://127.0.0.1:389"
user = "CN=admin,DC=example,DC=org"
pass = ""
user_pattern = "CN=%USER%,OU=people,DC=example,DC=org"

[redis]
uri = "redis://127.0.0.1/"
```

### Environment variables

Environment variables can be set with the same names as in the CLI arguments in uppercase using `_` instead of `-`, prefixed with `AUTH_`.

Example:

 - `--config <FILE> -> AUTH_CONFIG=<FILE>`
 - `--ldap-user-pattern <PATTERN> -> AUTH_LDAP_USER_PATTERN=<pattern>`
