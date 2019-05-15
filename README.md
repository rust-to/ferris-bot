# ferris-bot
A discord bot for rust servers. Created by the Toronto Rustaceans community.

## Usage
Start the bot by running `cargo run` in the directory root

**Some commands to get started:**

- `!help`
- `!say [message]`

## Installation

```shell
git clone https://github.com/rust-to/ferris-bot
cd ferris-bot
cargo build
cargo run
```

## License
This project is licensed under Apache 2.0 and a dual license with MIT may be added in the future.

*See the LICENSE file for details*

## Contributing
The details for contribution guidelines are still being worked out. A CONTRIBUTING.md will be added soon.

The guidelines so far are as follows:

- Commits and pull requests should already be formatted using cargo fmt
    - it is recommended to set up automatic format on save in your ide
- Code should be as modular as possible and split into minimal traits, modules (in their own files), and functions
- Follow the Semantic Versioning guidelines
- by committing code you agree that all code being added is written by you and is licensed under apache 2.0 / MIT
- All commits should be signed by the public key linked with your github account. *See https://help.github.com/en/articles/signing-commits*
- Repo will be split into multiple branches. Master has latest stable release, develop is bleeding-edge that we merge feature branches into