## Rust MCP Server 0.1.0
| 🟢 Tools (26) | 🟢 Prompts (1) | 🟢 Resources (95) | <span style="opacity:0.6">🔴 Logging</span> | <span style="opacity:0.6">🔴 Experimental</span> |
| --- | --- | --- | --- | --- |
## 🛠️ Tools (26)


- **cargo-add**
  - Adds a dependency to a Rust project using cargo add.
  - **Inputs:**
      - <code>branch</code> : string<br />
      - <code>build</code> : boolean<br />
      - <code>default_features</code> : boolean<br />
      - <code>dev</code> : boolean<br />
      - <code>dry_run</code> : boolean<br />
      - <code>features</code> : string [ ]<br />
      - <code>frozen</code> : boolean<br />
      - <code>git</code> : string<br />
      - <code>ignore_rust_version</code> : boolean<br />
      - <code>locked</code> : boolean<br />
      - <code>lockfile_path</code> : string<br />
      - <code>manifest_path</code> : string<br />
      - <code>no_default_features</code> : boolean<br />
      - <code>offline</code> : boolean<br />
      - <code>optional</code> : boolean<br />
      - <code>package</code> : string<br />
      - <code>path</code> : string<br />
      - <code>quiet</code> : boolean<br />
      - <code>registry</code> : string<br />
      - <code>rename</code> : string<br />
      - <code>rev</code> : string<br />
      - <code>tag</code> : string<br />
      - <code>target</code> : string<br />
      - <code>target_package</code> : string<br />
      - <code>toolchain</code> : string<br />
      - <code>verbose</code> : boolean<br />
      - <code>version</code> : string<br />

- **cargo-build**
  - Builds a Rust project using Cargo. Usually, run without any additional arguments.
  - **Inputs:**
      - <code>all_features</code> : boolean<br />
      - <code>all_targets</code> : boolean<br />
      - <code>bench</code> : string<br />
      - <code>benches</code> : boolean<br />
      - <code>bin</code> : string<br />
      - <code>bins</code> : boolean<br />
      - <code>example</code> : string<br />
      - <code>examples</code> : boolean<br />
      - <code>exclude</code> : string [ ]<br />
      - <code>features</code> : string [ ]<br />
      - <code>frozen</code> : boolean<br />
      - <code>ignore_rust_version</code> : boolean<br />
      - <code>jobs</code> : number<br />
      - <code>keep_going</code> : boolean<br />
      - <code>lib</code> : boolean<br />
      - <code>locked</code> : boolean<br />
      - <code>lockfile_path</code> : string<br />
      - <code>manifest_path</code> : string<br />
      - <code>no_default_features</code> : boolean<br />
      - <code>offline</code> : boolean<br />
      - <code>package</code> : string [ ]<br />
      - <code>profile</code> : string<br />
      - <code>quiet</code> : boolean<br />
      - <code>release</code> : boolean<br />
      - <code>target</code> : string<br />
      - <code>target_dir</code> : string<br />
      - <code>test</code> : string<br />
      - <code>tests</code> : boolean<br />
      - <code>toolchain</code> : string<br />
      - <code>verbose</code> : boolean<br />
      - <code>warnings_as_errors</code> : boolean<br />
      - <code>workspace</code> : boolean<br />

- **cargo-check**
  - Checks a Rust package and all of its dependencies for errors. Usually, run without any additional arguments.
  - **Inputs:**
      - <code>all_features</code> : boolean<br />
      - <code>all_targets</code> : boolean<br />
      - <code>bench</code> : string<br />
      - <code>benches</code> : boolean<br />
      - <code>bin</code> : string<br />
      - <code>bins</code> : boolean<br />
      - <code>example</code> : string<br />
      - <code>examples</code> : boolean<br />
      - <code>exclude</code> : string [ ]<br />
      - <code>features</code> : string [ ]<br />
      - <code>frozen</code> : boolean<br />
      - <code>ignore_rust_version</code> : boolean<br />
      - <code>jobs</code> : number<br />
      - <code>keep_going</code> : boolean<br />
      - <code>lib</code> : boolean<br />
      - <code>locked</code> : boolean<br />
      - <code>lockfile_path</code> : string<br />
      - <code>manifest_path</code> : string<br />
      - <code>no_default_features</code> : boolean<br />
      - <code>offline</code> : boolean<br />
      - <code>package</code> : string [ ]<br />
      - <code>profile</code> : string<br />
      - <code>quiet</code> : boolean<br />
      - <code>release</code> : boolean<br />
      - <code>target</code> : string<br />
      - <code>target_dir</code> : string<br />
      - <code>test</code> : string<br />
      - <code>tests</code> : boolean<br />
      - <code>toolchain</code> : string<br />
      - <code>verbose</code> : boolean<br />
      - <code>warnings_as_errors</code> : boolean<br />
      - <code>workspace</code> : boolean<br />

- **cargo-clean**
  - Cleans the target directory for a Rust project using Cargo. By default, it cleans the entire workspace.
  - **Inputs:**
      - <code>doc</code> : boolean<br />
      - <code>dry_run</code> : boolean<br />
      - <code>frozen</code> : boolean<br />
      - <code>locked</code> : boolean<br />
      - <code>lockfile_path</code> : string<br />
      - <code>manifest_path</code> : string<br />
      - <code>offline</code> : boolean<br />
      - <code>package</code> : string<br />
      - <code>profile</code> : string<br />
      - <code>quiet</code> : boolean<br />
      - <code>release</code> : boolean<br />
      - <code>target</code> : string<br />
      - <code>target_dir</code> : string<br />
      - <code>toolchain</code> : string<br />
      - <code>verbose</code> : boolean<br />

- **cargo-clippy**
  - Checks a Rust package to catch common mistakes and improve code quality using Clippy
  - **Inputs:**
      - <code>all_features</code> : boolean<br />
      - <code>all_targets</code> : boolean<br />
      - <code>allow_dirty</code> : boolean<br />
      - <code>bench</code> : string<br />
      - <code>benches</code> : boolean<br />
      - <code>bin</code> : string<br />
      - <code>bins</code> : boolean<br />
      - <code>example</code> : string<br />
      - <code>examples</code> : boolean<br />
      - <code>exclude</code> : string [ ]<br />
      - <code>features</code> : string [ ]<br />
      - <code>fix</code> : boolean<br />
      - <code>frozen</code> : boolean<br />
      - <code>ignore_rust_version</code> : boolean<br />
      - <code>lib</code> : boolean<br />
      - <code>locked</code> : boolean<br />
      - <code>manifest_path</code> : string<br />
      - <code>no_default_features</code> : boolean<br />
      - <code>no_deps</code> : boolean<br />
      - <code>offline</code> : boolean<br />
      - <code>package</code> : string [ ]<br />
      - <code>profile</code> : string<br />
      - <code>quiet</code> : boolean<br />
      - <code>release</code> : boolean<br />
      - <code>target</code> : string<br />
      - <code>target_dir</code> : string<br />
      - <code>test</code> : string<br />
      - <code>tests</code> : boolean<br />
      - <code>toolchain</code> : string<br />
      - <code>verbose</code> : boolean<br />
      - <code>warnings_as_errors</code> : boolean<br />
      - <code>workspace</code> : boolean<br />

- **cargo-deny-check**
  - Checks a project's crate graph for security advisories, license compliance, banned crates.
  - **Inputs:**
      - <code>all_features</code> : boolean<br />
      - <code>allow</code> : string [ ]<br />
      - <code>config</code> : string<br />
      - <code>deny</code> : string [ ]<br />
      - <code>disable_fetch</code> : boolean<br />
      - <code>exclude</code> : string [ ]<br />
      - <code>exclude_dev</code> : boolean<br />
      - <code>feature_depth</code> : number<br />
      - <code>features</code> : string [ ]<br />
      - <code>format</code> : string<br />
      - <code>graph</code> : string<br />
      - <code>hide_inclusion_graph</code> : boolean<br />
      - <code>locked</code> : boolean<br />
      - <code>log_level</code> : string<br />
      - <code>manifest_path</code> : string<br />
      - <code>no_default_features</code> : boolean<br />
      - <code>offline</code> : boolean<br />
      - <code>show_stats</code> : boolean<br />
      - <code>target</code> : string [ ]<br />
      - <code>warn</code> : string [ ]<br />
      - <code>which</code> : string [ ]<br />
      - <code>workspace</code> : boolean<br />

- **cargo-deny-init**
  - Creates a cargo-deny config from a template
  - **Inputs:**
      - <code>config</code> : string<br />

- **cargo-deny-install**
  - Installs cargo-deny tool for dependency graph analysis and security checks

- **cargo-deny-list**
  - Outputs a listing of all licenses and the crates that use them
  - **Inputs:**
      - <code>config</code> : string<br />
      - <code>format</code> : string<br />
      - <code>layout</code> : string<br />
      - <code>threshold</code> : number<br />

- **cargo-fmt**
  - Formats Rust code using rustfmt. Usually, run without any additional arguments.
  - **Inputs:**
      - <code>all</code> : boolean<br />
      - <code>check</code> : boolean<br />
      - <code>manifest_path</code> : string<br />
      - <code>message_format</code> : string<br />
      - <code>package</code> : string [ ]<br />
      - <code>quiet</code> : boolean<br />
      - <code>toolchain</code> : string<br />
      - <code>verbose</code> : boolean<br />

- **cargo-generate_lockfile**
  - Generates or updates the Cargo.lock file for a Rust project. Usually, run without any additional arguments.
  - **Inputs:**
      - <code>frozen</code> : boolean<br />
      - <code>ignore_rust_version</code> : boolean<br />
      - <code>locked</code> : boolean<br />
      - <code>lockfile_path</code> : string<br />
      - <code>manifest_path</code> : string<br />
      - <code>offline</code> : boolean<br />
      - <code>package</code> : string<br />
      - <code>quiet</code> : boolean<br />
      - <code>verbose</code> : boolean<br />

- **cargo-hack**
  - Cargo subcommand to provide various options useful for testing and continuous integration, including feature testing and multi-version compatibility. Available commands: check, test, build, clippy. Recommend using <code>check</code> for fast validation. Example: cargo-hack with "feature_powerset": true, "depth": 3, "keep_going": true
  - **Inputs:**
      - <code>clean_per_run</code> : boolean<br />
      - <code>clean_per_version</code> : boolean<br />
      - <code>command</code> : string<br />
      - <code>depth</code> : number<br />
      - <code>each_feature</code> : boolean<br />
      - <code>exclude</code> : string [ ]<br />
      - <code>exclude_all_features</code> : boolean<br />
      - <code>exclude_features</code> : string [ ]<br />
      - <code>exclude_no_default_features</code> : boolean<br />
      - <code>feature_powerset</code> : boolean<br />
      - <code>features</code> : string [ ]<br />
      - <code>group_features</code> : string [ ]<br />
      - <code>ignore_private</code> : boolean<br />
      - <code>ignore_unknown_features</code> : boolean<br />
      - <code>include_features</code> : string [ ]<br />
      - <code>keep_going</code> : boolean<br />
      - <code>locked</code> : boolean<br />
      - <code>log_group</code> : string<br />
      - <code>manifest_path</code> : string<br />
      - <code>mutually_exclusive_features</code> : string [ ]<br />
      - <code>no_dev_deps</code> : boolean<br />
      - <code>no_manifest_path</code> : boolean<br />
      - <code>no_private</code> : boolean<br />
      - <code>optional_deps</code> : string [ ]<br />
      - <code>package</code> : string [ ]<br />
      - <code>partition</code> : string<br />
      - <code>print_command_list</code> : boolean<br />
      - <code>remove_dev_deps</code> : boolean<br />
      - <code>rust_version</code> : boolean<br />
      - <code>target</code> : string [ ]<br />
      - <code>verbose</code> : boolean<br />
      - <code>version_range</code> : string<br />
      - <code>version_step</code> : number<br />
      - <code>workspace</code> : boolean<br />

- **cargo-hack-install**
  - Installs cargo-hack tool for feature testing and continuous integration

- **cargo-info**
  - Display information about a package. Information includes package description, list of available features, etc. Equivalent to 'cargo info <SPEC>'.
  - **Inputs:**
      - <code>config</code> : string<br />
      - <code>frozen</code> : boolean<br />
      - <code>index</code> : string<br />
      - <code>locked</code> : boolean<br />
      - <code>offline</code> : boolean<br />
      - <code>quiet</code> : boolean<br />
      - <code>registry</code> : string<br />
      - <code>spec</code> : string<br />
      - <code>verbose</code> : boolean<br />
      - <code>version</code> : string<br />

- **cargo-list**
  - Lists installed cargo commands using 'cargo --list'.

- **cargo-machete**
  - Finds unused dependencies in a fast yet imprecise way. Helps identify dependencies that are declared in Cargo.toml but not actually used in the code.
  - **Inputs:**
      - <code>fix</code> : boolean<br />
      - <code>no_ignore</code> : boolean<br />
      - <code>paths</code> : string [ ]<br />
      - <code>skip_target_dir</code> : boolean<br />
      - <code>with_metadata</code> : boolean<br />

- **cargo-machete-install**
  - Installs cargo-machete tool for finding unused dependencies

- **cargo-metadata**
  - Outputs a listing of a project's resolved dependencies and metadata in machine-readable format (JSON).
  - **Inputs:**
      - <code>all_features</code> : boolean<br />
      - <code>config</code> : string<br />
      - <code>features</code> : string<br />
      - <code>filter_platform</code> : string<br />
      - <code>frozen</code> : boolean<br />
      - <code>locked</code> : boolean<br />
      - <code>lockfile_path</code> : string<br />
      - <code>manifest_path</code> : string<br />
      - <code>no_default_features</code> : boolean<br />
      - <code>no_deps</code> : boolean<br />
      - <code>offline</code> : boolean<br />
      - <code>quiet</code> : boolean<br />
      - <code>toolchain</code> : string<br />
      - <code>verbose</code> : boolean<br />

- **cargo-new**
  - Create a new cargo package at <path>. Creates a new Rust project with the specified name and template.
  - **Inputs:**
      - <code>bin</code> : boolean<br />
      - <code>edition</code> : string<br />
      - <code>frozen</code> : boolean<br />
      - <code>lib</code> : boolean<br />
      - <code>locked</code> : boolean<br />
      - <code>name</code> : string<br />
      - <code>offline</code> : boolean<br />
      - <code>path</code> : string<br />
      - <code>quiet</code> : boolean<br />
      - <code>registry</code> : string<br />
      - <code>toolchain</code> : string<br />
      - <code>vcs</code> : string<br />
      - <code>verbose</code> : boolean<br />

- **cargo-remove**
  - Remove dependencies from a Cargo.toml manifest file.
  - **Inputs:**
      - <code>build</code> : boolean<br />
      - <code>dep_id</code> : string [ ]<br />
      - <code>dev</code> : boolean<br />
      - <code>dry_run</code> : boolean<br />
      - <code>frozen</code> : boolean<br />
      - <code>locked</code> : boolean<br />
      - <code>lockfile_path</code> : string<br />
      - <code>manifest_path</code> : string<br />
      - <code>offline</code> : boolean<br />
      - <code>package</code> : string<br />
      - <code>quiet</code> : boolean<br />
      - <code>target</code> : string<br />
      - <code>toolchain</code> : string<br />
      - <code>verbose</code> : boolean<br />

- **cargo-search**
  - Search packages in the registry. Default registry is crates.io. Equivalent to 'cargo search <code>QUERY</code>'.
  - **Inputs:**
      - <code>limit</code> : number<br />
      - <code>query</code> : string<br />
      - <code>quiet</code> : boolean<br />
      - <code>registry</code> : string<br />

- **cargo-test**
  - Run `cargo test` to execute Rust tests in the current project.
  - **Inputs:**
      - <code>all_features</code> : boolean<br />
      - <code>all_targets</code> : boolean<br />
      - <code>bench</code> : string<br />
      - <code>benches</code> : boolean<br />
      - <code>bin</code> : string<br />
      - <code>bins</code> : boolean<br />
      - <code>doc</code> : boolean<br />
      - <code>example</code> : string<br />
      - <code>examples</code> : boolean<br />
      - <code>exclude</code> : string [ ]<br />
      - <code>features</code> : string<br />
      - <code>frozen</code> : boolean<br />
      - <code>ignore_rust_version</code> : boolean<br />
      - <code>jobs</code> : number<br />
      - <code>lib</code> : boolean<br />
      - <code>locked</code> : boolean<br />
      - <code>lockfile_path</code> : string<br />
      - <code>manifest_path</code> : string<br />
      - <code>no_default_features</code> : boolean<br />
      - <code>no_fail_fast</code> : boolean<br />
      - <code>no_run</code> : boolean<br />
      - <code>offline</code> : boolean<br />
      - <code>package</code> : string<br />
      - <code>profile</code> : string<br />
      - <code>quiet</code> : boolean<br />
      - <code>release</code> : boolean<br />
      - <code>target</code> : string<br />
      - <code>target_dir</code> : string<br />
      - <code>test</code> : string<br />
      - <code>test_args</code> : string [ ]<br />
      - <code>testname</code> : string<br />
      - <code>tests</code> : boolean<br />
      - <code>toolchain</code> : string<br />
      - <code>verbose</code> : boolean<br />
      - <code>workspace</code> : boolean<br />

- **cargo-update**
  - Update dependencies as recorded in the local lock file. Updates the dependencies in Cargo.lock to their latest compatible versions.
  - **Inputs:**
      - <code>breaking</code> : boolean<br />
      - <code>color</code> : string<br />
      - <code>config</code> : string<br />
      - <code>dry_run</code> : boolean<br />
      - <code>frozen</code> : boolean<br />
      - <code>ignore_rust_version</code> : boolean<br />
      - <code>locked</code> : boolean<br />
      - <code>lockfile_path</code> : string<br />
      - <code>manifest_path</code> : string<br />
      - <code>offline</code> : boolean<br />
      - <code>precise</code> : string<br />
      - <code>quiet</code> : boolean<br />
      - <code>recursive</code> : boolean<br />
      - <code>spec</code> : string [ ]<br />
      - <code>toolchain</code> : string<br />
      - <code>unstable_flags</code> : string [ ]<br />
      - <code>verbose</code> : boolean<br />
      - <code>workspace</code> : boolean<br />

- **rustup-show**
  - Show the active and installed toolchains or profiles. Shows the name of the active toolchain and the version of rustc. If the active toolchain has installed support for additional compilation targets, then they are listed as well.
  - **Inputs:**
      - <code>verbose</code> : boolean<br />

- **rustup-toolchain-add**
  - Install or update the given toolchains, or by default the active toolchain. Toolchain name can be <code>stable</code>, <code>nightly</code>, or a specific version like '1.8.0'.
  - **Inputs:**
      - <code>allow_downgrade</code> : boolean<br />
      - <code>components</code> : string [ ]<br />
      - <code>force</code> : boolean<br />
      - <code>force_non_host</code> : boolean<br />
      - <code>no_self_update</code> : boolean<br />
      - <code>profile</code> : string<br />
      - <code>targets</code> : string [ ]<br />
      - <code>toolchain</code> : string<br />

- **rustup-update**
  - Update Rust toolchains and rustup. With no toolchain specified, updates each of the installed toolchains from the official release channels, then updates rustup itself. If given a toolchain argument then updates that toolchain.
  - **Inputs:**
      - <code>force</code> : boolean<br />
      - <code>force_non_host</code> : boolean<br />
      - <code>no_self_update</code> : boolean<br />
      - <code>toolchain</code> : string<br />


## 📝 Prompts (1)


- **rustup-update-toolset**
  - Provides instruction on how to update Rust toolset

## 📄 Resources (95)


- **Changelog**
  - Cargo Book: Changelog
  - URI: <a>cargo-book://CHANGELOG.md</a> <i>(text/markdown)</i>

- **Appendix: Git Authentication**
  - Cargo Book: Appendix: Git Authentication
  - URI: <a>cargo-book://appendix/git-authentication.md</a> <i>(text/markdown)</i>

- **Appendix: Glossary**
  - Cargo Book: Appendix: Glossary
  - URI: <a>cargo-book://appendix/glossary.md</a> <i>(text/markdown)</i>

- **Build Commands**
  - Cargo Book: Build Commands
  - URI: <a>cargo-book://commands/build-commands.md</a> <i>(text/markdown)</i>

- **cargo add**
  - Cargo Book: cargo add
  - URI: <a>cargo-book://commands/cargo-add.md</a> <i>(text/markdown)</i>

- **cargo bench**
  - Cargo Book: cargo bench
  - URI: <a>cargo-book://commands/cargo-bench.md</a> <i>(text/markdown)</i>

- **cargo build**
  - Cargo Book: cargo build
  - URI: <a>cargo-book://commands/cargo-build.md</a> <i>(text/markdown)</i>

- **cargo check**
  - Cargo Book: cargo check
  - URI: <a>cargo-book://commands/cargo-check.md</a> <i>(text/markdown)</i>

- **cargo clean**
  - Cargo Book: cargo clean
  - URI: <a>cargo-book://commands/cargo-clean.md</a> <i>(text/markdown)</i>

- **cargo clippy**
  - Cargo Book: cargo clippy
  - URI: <a>cargo-book://commands/cargo-clippy.md</a> <i>(text/markdown)</i>

- **cargo doc**
  - Cargo Book: cargo doc
  - URI: <a>cargo-book://commands/cargo-doc.md</a> <i>(text/markdown)</i>

- **cargo fetch**
  - Cargo Book: cargo fetch
  - URI: <a>cargo-book://commands/cargo-fetch.md</a> <i>(text/markdown)</i>

- **cargo fix**
  - Cargo Book: cargo fix
  - URI: <a>cargo-book://commands/cargo-fix.md</a> <i>(text/markdown)</i>

- **cargo fmt**
  - Cargo Book: cargo fmt
  - URI: <a>cargo-book://commands/cargo-fmt.md</a> <i>(text/markdown)</i>

- **cargo generate-lockfile**
  - Cargo Book: cargo generate-lockfile
  - URI: <a>cargo-book://commands/cargo-generate-lockfile.md</a> <i>(text/markdown)</i>

- **cargo help**
  - Cargo Book: cargo help
  - URI: <a>cargo-book://commands/cargo-help.md</a> <i>(text/markdown)</i>

- **cargo info**
  - Cargo Book: cargo info
  - URI: <a>cargo-book://commands/cargo-info.md</a> <i>(text/markdown)</i>

- **cargo init**
  - Cargo Book: cargo init
  - URI: <a>cargo-book://commands/cargo-init.md</a> <i>(text/markdown)</i>

- **cargo install**
  - Cargo Book: cargo install
  - URI: <a>cargo-book://commands/cargo-install.md</a> <i>(text/markdown)</i>

- **cargo locate-project**
  - Cargo Book: cargo locate-project
  - URI: <a>cargo-book://commands/cargo-locate-project.md</a> <i>(text/markdown)</i>

- **cargo login**
  - Cargo Book: cargo login
  - URI: <a>cargo-book://commands/cargo-login.md</a> <i>(text/markdown)</i>

- **cargo logout**
  - Cargo Book: cargo logout
  - URI: <a>cargo-book://commands/cargo-logout.md</a> <i>(text/markdown)</i>

- **cargo metadata**
  - Cargo Book: cargo metadata
  - URI: <a>cargo-book://commands/cargo-metadata.md</a> <i>(text/markdown)</i>

- **cargo miri**
  - Cargo Book: cargo miri
  - URI: <a>cargo-book://commands/cargo-miri.md</a> <i>(text/markdown)</i>

- **cargo new**
  - Cargo Book: cargo new
  - URI: <a>cargo-book://commands/cargo-new.md</a> <i>(text/markdown)</i>

- **cargo owner**
  - Cargo Book: cargo owner
  - URI: <a>cargo-book://commands/cargo-owner.md</a> <i>(text/markdown)</i>

- **cargo package**
  - Cargo Book: cargo package
  - URI: <a>cargo-book://commands/cargo-package.md</a> <i>(text/markdown)</i>

- **cargo pkgid**
  - Cargo Book: cargo pkgid
  - URI: <a>cargo-book://commands/cargo-pkgid.md</a> <i>(text/markdown)</i>

- **cargo publish**
  - Cargo Book: cargo publish
  - URI: <a>cargo-book://commands/cargo-publish.md</a> <i>(text/markdown)</i>

- **cargo remove**
  - Cargo Book: cargo remove
  - URI: <a>cargo-book://commands/cargo-remove.md</a> <i>(text/markdown)</i>

- **cargo report**
  - Cargo Book: cargo report
  - URI: <a>cargo-book://commands/cargo-report.md</a> <i>(text/markdown)</i>

- **cargo run**
  - Cargo Book: cargo run
  - URI: <a>cargo-book://commands/cargo-run.md</a> <i>(text/markdown)</i>

- **cargo rustc**
  - Cargo Book: cargo rustc
  - URI: <a>cargo-book://commands/cargo-rustc.md</a> <i>(text/markdown)</i>

- **cargo rustdoc**
  - Cargo Book: cargo rustdoc
  - URI: <a>cargo-book://commands/cargo-rustdoc.md</a> <i>(text/markdown)</i>

- **cargo search**
  - Cargo Book: cargo search
  - URI: <a>cargo-book://commands/cargo-search.md</a> <i>(text/markdown)</i>

- **cargo test**
  - Cargo Book: cargo test
  - URI: <a>cargo-book://commands/cargo-test.md</a> <i>(text/markdown)</i>

- **cargo tree**
  - Cargo Book: cargo tree
  - URI: <a>cargo-book://commands/cargo-tree.md</a> <i>(text/markdown)</i>

- **cargo uninstall**
  - Cargo Book: cargo uninstall
  - URI: <a>cargo-book://commands/cargo-uninstall.md</a> <i>(text/markdown)</i>

- **cargo update**
  - Cargo Book: cargo update
  - URI: <a>cargo-book://commands/cargo-update.md</a> <i>(text/markdown)</i>

- **cargo vendor**
  - Cargo Book: cargo vendor
  - URI: <a>cargo-book://commands/cargo-vendor.md</a> <i>(text/markdown)</i>

- **cargo version**
  - Cargo Book: cargo version
  - URI: <a>cargo-book://commands/cargo-version.md</a> <i>(text/markdown)</i>

- **cargo yank**
  - Cargo Book: cargo yank
  - URI: <a>cargo-book://commands/cargo-yank.md</a> <i>(text/markdown)</i>

- **cargo**
  - Cargo Book: cargo
  - URI: <a>cargo-book://commands/cargo.md</a> <i>(text/markdown)</i>

- **Deprecated and Removed Commands**
  - Cargo Book: Deprecated and Removed Commands
  - URI: <a>cargo-book://commands/deprecated-and-removed.md</a> <i>(text/markdown)</i>

- **General Commands**
  - Cargo Book: General Commands
  - URI: <a>cargo-book://commands/general-commands.md</a> <i>(text/markdown)</i>

- **Cargo Commands**
  - Cargo Book: Cargo Commands
  - URI: <a>cargo-book://commands/index.md</a> <i>(text/markdown)</i>

- **Manifest Commands**
  - Cargo Book: Manifest Commands
  - URI: <a>cargo-book://commands/manifest-commands.md</a> <i>(text/markdown)</i>

- **Package Commands**
  - Cargo Book: Package Commands
  - URI: <a>cargo-book://commands/package-commands.md</a> <i>(text/markdown)</i>

- **Publishing Commands**
  - Cargo Book: Publishing Commands
  - URI: <a>cargo-book://commands/publishing-commands.md</a> <i>(text/markdown)</i>

- **FAQ**
  - Cargo Book: FAQ
  - URI: <a>cargo-book://faq.md</a> <i>(text/markdown)</i>

- **First Steps with Cargo**
  - Cargo Book: First Steps with Cargo
  - URI: <a>cargo-book://getting-started/first-steps.md</a> <i>(text/markdown)</i>

- **Getting Started with Cargo**
  - Cargo Book: Getting Started with Cargo
  - URI: <a>cargo-book://getting-started/index.md</a> <i>(text/markdown)</i>

- **Installing Cargo**
  - Cargo Book: Installing Cargo
  - URI: <a>cargo-book://getting-started/installation.md</a> <i>(text/markdown)</i>

- **Cargo Home**
  - Cargo Book: Cargo Home
  - URI: <a>cargo-book://guide/cargo-home.md</a> <i>(text/markdown)</i>

- **Cargo.toml vs Cargo.lock**
  - Cargo Book: Cargo.toml vs Cargo.lock
  - URI: <a>cargo-book://guide/cargo-toml-vs-cargo-lock.md</a> <i>(text/markdown)</i>

- **Continuous Integration**
  - Cargo Book: Continuous Integration
  - URI: <a>cargo-book://guide/continuous-integration.md</a> <i>(text/markdown)</i>

- **Creating a New Package**
  - Cargo Book: Creating a New Package
  - URI: <a>cargo-book://guide/creating-a-new-project.md</a> <i>(text/markdown)</i>

- **Dependencies**
  - Cargo Book: Dependencies
  - URI: <a>cargo-book://guide/dependencies.md</a> <i>(text/markdown)</i>

- **Cargo Guide**
  - Cargo Book: Cargo Guide
  - URI: <a>cargo-book://guide/index.md</a> <i>(text/markdown)</i>

- **Package Layout**
  - Cargo Book: Package Layout
  - URI: <a>cargo-book://guide/project-layout.md</a> <i>(text/markdown)</i>

- **Tests**
  - Cargo Book: Tests
  - URI: <a>cargo-book://guide/tests.md</a> <i>(text/markdown)</i>

- **Why Cargo Exists**
  - Cargo Book: Why Cargo Exists
  - URI: <a>cargo-book://guide/why-cargo-exists.md</a> <i>(text/markdown)</i>

- **Working on an Existing Package**
  - Cargo Book: Working on an Existing Package
  - URI: <a>cargo-book://guide/working-on-an-existing-project.md</a> <i>(text/markdown)</i>

- **Introduction to Cargo**
  - Cargo Book: Introduction to Cargo
  - URI: <a>cargo-book://index.md</a> <i>(text/markdown)</i>

- **Build Cache**
  - Cargo Book: Build Cache
  - URI: <a>cargo-book://reference/build-cache.md</a> <i>(text/markdown)</i>

- **Build Script Examples**
  - Cargo Book: Build Script Examples
  - URI: <a>cargo-book://reference/build-script-examples.md</a> <i>(text/markdown)</i>

- **Build Scripts**
  - Cargo Book: Build Scripts
  - URI: <a>cargo-book://reference/build-scripts.md</a> <i>(text/markdown)</i>

- **Cargo Targets**
  - Cargo Book: Cargo Targets
  - URI: <a>cargo-book://reference/cargo-targets.md</a> <i>(text/markdown)</i>

- **Configuration**
  - Cargo Book: Configuration
  - URI: <a>cargo-book://reference/config.md</a> <i>(text/markdown)</i>

- **Credential Provider Protocol**
  - Cargo Book: Credential Provider Protocol
  - URI: <a>cargo-book://reference/credential-provider-protocol.md</a> <i>(text/markdown)</i>

- **Environment Variables**
  - Cargo Book: Environment Variables
  - URI: <a>cargo-book://reference/environment-variables.md</a> <i>(text/markdown)</i>

- **External Tools**
  - Cargo Book: External Tools
  - URI: <a>cargo-book://reference/external-tools.md</a> <i>(text/markdown)</i>

- **Features Examples**
  - Cargo Book: Features Examples
  - URI: <a>cargo-book://reference/features-examples.md</a> <i>(text/markdown)</i>

- **Features**
  - Cargo Book: Features
  - URI: <a>cargo-book://reference/features.md</a> <i>(text/markdown)</i>

- **Future incompat report**
  - Cargo Book: Future incompat report
  - URI: <a>cargo-book://reference/future-incompat-report.md</a> <i>(text/markdown)</i>

- **Cargo Reference**
  - Cargo Book: Cargo Reference
  - URI: <a>cargo-book://reference/index.md</a> <i>(text/markdown)</i>

- **Lints**
  - Cargo Book: Lints
  - URI: <a>cargo-book://reference/lints.md</a> <i>(text/markdown)</i>

- **The Manifest Format**
  - Cargo Book: The Manifest Format
  - URI: <a>cargo-book://reference/manifest.md</a> <i>(text/markdown)</i>

- **Overriding Dependencies**
  - Cargo Book: Overriding Dependencies
  - URI: <a>cargo-book://reference/overriding-dependencies.md</a> <i>(text/markdown)</i>

- **Package ID Specifications**
  - Cargo Book: Package ID Specifications
  - URI: <a>cargo-book://reference/pkgid-spec.md</a> <i>(text/markdown)</i>

- **Profiles**
  - Cargo Book: Profiles
  - URI: <a>cargo-book://reference/profiles.md</a> <i>(text/markdown)</i>

- **Publishing on crates.io**
  - Cargo Book: Publishing on crates.io
  - URI: <a>cargo-book://reference/publishing.md</a> <i>(text/markdown)</i>

- **Registries**
  - Cargo Book: Registries
  - URI: <a>cargo-book://reference/registries.md</a> <i>(text/markdown)</i>

- **Registry Authentication**
  - Cargo Book: Registry Authentication
  - URI: <a>cargo-book://reference/registry-authentication.md</a> <i>(text/markdown)</i>

- **Registry Index**
  - Cargo Book: Registry Index
  - URI: <a>cargo-book://reference/registry-index.md</a> <i>(text/markdown)</i>

- **Registry Web API**
  - Cargo Book: Registry Web API
  - URI: <a>cargo-book://reference/registry-web-api.md</a> <i>(text/markdown)</i>

- **Dependency Resolution**
  - Cargo Book: Dependency Resolution
  - URI: <a>cargo-book://reference/resolver.md</a> <i>(text/markdown)</i>

- **Running a Registry**
  - Cargo Book: Running a Registry
  - URI: <a>cargo-book://reference/running-a-registry.md</a> <i>(text/markdown)</i>

- **Rust Version**
  - Cargo Book: Rust Version
  - URI: <a>cargo-book://reference/rust-version.md</a> <i>(text/markdown)</i>

- **SemVer Compatibility**
  - Cargo Book: SemVer Compatibility
  - URI: <a>cargo-book://reference/semver.md</a> <i>(text/markdown)</i>

- **Source Replacement**
  - Cargo Book: Source Replacement
  - URI: <a>cargo-book://reference/source-replacement.md</a> <i>(text/markdown)</i>

- **Specifying Dependencies**
  - Cargo Book: Specifying Dependencies
  - URI: <a>cargo-book://reference/specifying-dependencies.md</a> <i>(text/markdown)</i>

- **Reporting build timings**
  - Cargo Book: Reporting build timings
  - URI: <a>cargo-book://reference/timings.md</a> <i>(text/markdown)</i>

- **Unstable Features**
  - Cargo Book: Unstable Features
  - URI: <a>cargo-book://reference/unstable.md</a> <i>(text/markdown)</i>

- **Workspaces**
  - Cargo Book: Workspaces
  - URI: <a>cargo-book://reference/workspaces.md</a> <i>(text/markdown)</i>


<sup>◾ generated by [mcp-discovery](https://github.com/rust-mcp-stack/mcp-discovery)</sup>
