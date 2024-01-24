```txt
Compile a local package and all of its dependencies

Usage: cargo.exe build [OPTIONS]

Options:
      --future-incompat-report   Outputs a future incompatibility report at the end of the build
      --message-format <FMT>     Error format
  -v, --verbose...               Use verbose output (-vv very verbose/build.rs output)
  -q, --quiet                    Do not print cargo log messages
      --color <WHEN>             Coloring: auto, always, never
      --config <KEY=VALUE|PATH>  Override a configuration value
  -Z <FLAG>                      Unstable (nightly-only) flags to Cargo, see 'cargo -Z help' for details
  -h, --help                     Print help

Package Selection:
  -p, --package [<SPEC>]  Package to build (see `cargo help pkgid`)
      --workspace         Build all packages in the workspace
      --exclude <SPEC>    Exclude packages from the build
      --all               Alias for --workspace (deprecated)

Target Selection:
      --lib               Build only this package's library
      --bins              Build all binaries
      --bin [<NAME>]      Build only the specified binary
      --examples          Build all examples
      --example [<NAME>]  Build only the specified example
      --tests             Build all targets that have `test = true` set
      --test [<NAME>]     Build only the specified test target
      --benches           Build all targets that have `bench = true` set
      --bench [<NAME>]    Build only the specified bench target
      --all-targets       Build all targets

Feature Selection:
  -F, --features <FEATURES>  Space or comma separated list of features to activate
      --all-features         Activate all available features
      --no-default-features  Do not activate the `default` feature

Compilation Options:
  -r, --release                 Build artifacts in release mode, with optimizations
      --profile <PROFILE-NAME>  Build artifacts with the specified profile
  -j, --jobs <N>                Number of parallel jobs, defaults to # of CPUs.
      --keep-going              Do not abort the build as soon as there is an error
      --target [<TRIPLE>]       Build for the target triple
      --target-dir <DIRECTORY>  Directory for all generated artifacts
      --artifact-dir <PATH>     Copy final artifacts to this directory (unstable)
      --build-plan              Output the build plan in JSON (unstable)
      --unit-graph              Output build graph in JSON (unstable)
      --timings[=<FMTS>]        Timing output formats (unstable) (comma separated): html, json

Manifest Options:
      --manifest-path <PATH>  Path to Cargo.toml
      --lockfile-path <PATH>  Path to Cargo.lock (unstable)
      --ignore-rust-version   Ignore `rust-version` specification in packages
      --locked                Assert that `Cargo.lock` will remain unchanged
      --offline               Run without accessing the network
      --frozen                Equivalent to specifying both --locked and --offline

Run `cargo help build` for more detailed information.
```