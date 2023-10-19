# Elastio Weather CLI

## Project Specifications

**Read-Only Files**
- .env
- .gitignore

**Environment**  
First of all you have to make some required installations. To make it relevant to your OS please visit official websites: <br>

1) Rust programming language - https://www.rust-lang.org/tools/install

2) Redis - https://redis.io/docs/getting-started/installation/

Then, please install all dependencies using `cargo` following commands

```bash
cargo clean

cargo install --path ./weather-app
```
After that you have to set global env vaiables: <br>
1) for Linux/MacOS use 
```bash
export $(grep -v '^#' weather-app/.env | xargs)
```
2) For Windows 
```bash
$env:PATH = (Get-Content weather-app/.env)
```

**Usage Example**

To get all basic commands you can `--help` subcomand
Example:

```bash
weather --help


weather get --help


weather configuration --help
```
Before getting current forecast, please configure weather provider. To do this execute following command:
```bash
weather configuration <PROVIDER>
```

Also, to see available providers, you can use:
```bash
weather providers
```
To get weather forecast from chosen provider you should execute:

Also, to see available providers, you can use:
```bash
weather get <ADDRESS> [DATE]
```
**MUST corresponded patterns**
1) Address - `City,Country` or `City,CountryCode` without whitespaces
2) Date - `Y-m-d` without whitespaces
