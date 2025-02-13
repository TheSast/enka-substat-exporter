# enka-substat-exporter

## Building

Nix (the package manager) is the only dependency, it will handle all other necessary packages, if you are not using it then check the list at L36-L50 of [./flake.nix](https://github.com/TheSast/enka-substat-exporter/blob/main/flake.nix#L36-L50) and figure it out for your system.

1. [Install nix](https://nix.dev/install-nix.html)
2. `nix develop`
3. `sh ./update_local_data.sh`
4. `cargo run`

## Usage

usage: [api link to non-profile or specific hoyo account on a profile] [optional prefix to filter builds on profile]
example: `cargo run -- "https://enka.network/api/profile/TheSast/hoyos/3OzWze/builds/" "2025-02"`
