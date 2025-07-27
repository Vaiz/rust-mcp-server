## Rust MCP Server 0.2.1
| 🟢 Tools (28) | 🟢 Prompts (1) | 🟢 Resources (95) | <span style="opacity:0.6">🔴 Logging</span> | <span style="opacity:0.6">🔴 Experimental</span> |
| --- | --- | --- | --- | --- |
## 🛠️ Tools (28)


- **cargo-add**
  - Adds a dependency to a Rust project using cargo add.
  - **Inputs:**
      - <code>branch</code> : string<br />
      - <code>default_features</code> : boolean<br />
      - <code>dependency_type</code> : string<br />
      - <code>dry_run</code> : boolean<br />
      - <code>features</code> : string [ ]<br />
      - <code>git</code> : string<br />
      - <code>ignore_rust_version</code> : boolean<br />
      - <code>lockfile_path</code> : string<br />
      - <code>locking_mode</code> : string<br />
      - <code>manifest_path</code> : string<br />
      - <code>no_default_features</code> : boolean<br />
      - <code>optional</code> : boolean<br />
      - <code>output_verbosity</code> : string<br />
      - <code>package</code> : string<br />
      - <code>path</code> : string<br />
      - <code>registry</code> : string<br />
      - <code>rename</code> : string<br />
      - <code>rev</code> : string<br />
      - <code>tag</code> : string<br />
      - <code>target</code> : string<br />
      - <code>target_package</code> : string<br />
      - <code>toolchain</code> : string<br />
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
      - <code>ignore_rust_version</code> : boolean<br />
      - <code>jobs</code> : number<br />
      - <code>keep_going</code> : boolean<br />
      - <code>lib</code> : boolean<br />
      - <code>lockfile_path</code> : string<br />
      - <code>locking_mode</code> : string<br />
      - <code>manifest_path</code> : string<br />
      - <code>no_default_features</code> : boolean<br />
      - <code>output_verbosity</code> : string<br />
      - <code>package</code> : string [ ]<br />
      - <code>profile</code> : string<br />
      - <code>release</code> : boolean<br />
      - <code>target</code> : string<br />
      - <code>target_dir</code> : string<br />
      - <code>test</code> : string<br />
      - <code>tests</code> : boolean<br />
      - <code>toolchain</code> : string<br />
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
      - <code>ignore_rust_version</code> : boolean<br />
      - <code>jobs</code> : number<br />
      - <code>keep_going</code> : boolean<br />
      - <code>lib</code> : boolean<br />
      - <code>lockfile_path</code> : string<br />
      - <code>locking_mode</code> : string<br />
      - <code>manifest_path</code> : string<br />
      - <code>no_default_features</code> : boolean<br />
      - <code>output_verbosity</code> : string<br />
      - <code>package</code> : string [ ]<br />
      - <code>profile</code> : string<br />
      - <code>release</code> : boolean<br />
      - <code>target</code> : string<br />
      - <code>target_dir</code> : string<br />
      - <code>test</code> : string<br />
      - <code>tests</code> : boolean<br />
      - <code>toolchain</code> : string<br />
      - <code>warnings_as_errors</code> : boolean<br />
      - <code>workspace</code> : boolean<br />

- **cargo-clean**
  - Cleans the target directory for a Rust project using Cargo. By default, it cleans the entire workspace.
  - **Inputs:**
      - <code>doc</code> : boolean<br />
      - <code>dry_run</code> : boolean<br />
      - <code>lockfile_path</code> : string<br />
      - <code>locking_mode</code> : string<br />
      - <code>manifest_path</code> : string<br />
      - <code>output_verbosity</code> : string<br />
      - <code>package</code> : string<br />
      - <code>profile</code> : string<br />
      - <code>release</code> : boolean<br />
      - <code>target</code> : string<br />
      - <code>target_dir</code> : string<br />
      - <code>toolchain</code> : string<br />

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
      - <code>ignore_rust_version</code> : boolean<br />
      - <code>lib</code> : boolean<br />
      - <code>locking_mode</code> : string<br />
      - <code>manifest_path</code> : string<br />
      - <code>no_default_features</code> : boolean<br />
      - <code>no_deps</code> : boolean<br />
      - <code>output_verbosity</code> : string<br />
      - <code>package</code> : string [ ]<br />
      - <code>profile</code> : string<br />
      - <code>release</code> : boolean<br />
      - <code>target</code> : string<br />
      - <code>target_dir</code> : string<br />
      - <code>test</code> : string<br />
      - <code>tests</code> : boolean<br />
      - <code>toolchain</code> : string<br />
      - <code>warnings_as_errors</code> : boolean<br />
      - <code>workspace</code> : boolean<br />

- **cargo-deny-check**
  - Checks a project's crate graph for security advisories, license compliance, banned crates.
  - **Inputs:**
      - <code>all_features</code> : boolean<br />
      - <code>allow</code> : string [ ]<br />
      - <code>allow_git_index</code> : boolean<br />
      - <code>audit_compatible_output</code> : boolean<br />
      - <code>config</code> : string<br />
      - <code>deny</code> : string [ ]<br />
      - <code>disable_fetch</code> : boolean<br />
      - <code>exclude</code> : string [ ]<br />
      - <code>exclude_dev</code> : boolean<br />
      - <code>exclude_unpublished</code> : boolean<br />
      - <code>feature_depth</code> : integer<br />
      - <code>features</code> : string [ ]<br />
      - <code>format</code> : string<br />
      - <code>graph</code> : string<br />
      - <code>hide_inclusion_graph</code> : boolean<br />
      - <code>locking_mode</code> : string<br />
      - <code>log_level</code> : string<br />
      - <code>manifest_path</code> : string<br />
      - <code>no_default_features</code> : boolean<br />
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
      - <code>output_verbosity</code> : string<br />
      - <code>package</code> : string [ ]<br />
      - <code>toolchain</code> : string<br />

- **cargo-generate_lockfile**
  - Generates or updates the Cargo.lock file for a Rust project. Usually, run without any additional arguments.
  - **Inputs:**
      - <code>ignore_rust_version</code> : boolean<br />
      - <code>lockfile_path</code> : string<br />
      - <code>locking_mode</code> : string<br />
      - <code>manifest_path</code> : string<br />
      - <code>output_verbosity</code> : string<br />
      - <code>package</code> : string<br />

- **cargo-hack**
  - Cargo subcommand to provide various options useful for testing and continuous integration, including feature testing and multi-version compatibility. Available commands: check, test, build, clippy. Recommend using <code>check</code> for fast validation. Example: cargo-hack with "feature_powerset": true, "depth": 3, "keep_going": true
  - **Inputs:**
      - <code>clean_per_run</code> : boolean<br />
      - <code>clean_per_version</code> : boolean<br />
      - <code>command</code> : string<br />
      - <code>depth</code> : integer<br />
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
      - <code>output_verbosity</code> : string<br />
      - <code>package</code> : string [ ]<br />
      - <code>partition</code> : string<br />
      - <code>print_command_list</code> : boolean<br />
      - <code>remove_dev_deps</code> : boolean<br />
      - <code>rust_version</code> : boolean<br />
      - <code>target</code> : string [ ]<br />
      - <code>version_range</code> : string<br />
      - <code>version_step</code> : integer<br />
      - <code>workspace</code> : boolean<br />

- **cargo-hack-install**
  - Installs cargo-hack tool for feature testing and continuous integration

- **cargo-info**
  - Display information about a package. Information includes package description, list of available features, etc. Equivalent to 'cargo info <SPEC>'.
  - **Inputs:**
      - <code>config</code> : string<br />
      - <code>index</code> : string<br />
      - <code>locking_mode</code> : string<br />
      - <code>output_verbosity</code> : string<br />
      - <code>package</code> : string<br />
      - <code>registry</code> : string<br />
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
      - <code>lockfile_path</code> : string<br />
      - <code>locking_mode</code> : string<br />
      - <code>manifest_path</code> : string<br />
      - <code>no_default_features</code> : boolean<br />
      - <code>no_deps</code> : boolean<br />
      - <code>output_verbosity</code> : string<br />
      - <code>toolchain</code> : string<br />

- **cargo-new**
  - Create a new cargo package at <path>. Creates a new Rust project with the specified name and template.
  - **Inputs:**
      - <code>bin</code> : boolean<br />
      - <code>edition</code> : string<br />
      - <code>lib</code> : boolean<br />
      - <code>locking_mode</code> : string<br />
      - <code>name</code> : string<br />
      - <code>output_verbosity</code> : string<br />
      - <code>path</code> : string<br />
      - <code>registry</code> : string<br />
      - <code>toolchain</code> : string<br />
      - <code>vcs</code> : string<br />

- **cargo-package**
  - Assemble the local package into a distributable tarball for publishing or distribution. <br/>    <br/>    Common use cases:<br/>    - Create a .crate file for publishing to crates.io or a private registry<br/>    - Generate distribution packages for deployment or sharing<br/>    - Validate package contents before publishing (using --list)<br/>    - Test packaging process without verification (using --no-verify)<br/>    - Package workspace members selectively or all at once<br/>    <br/>    The generated tarball contains all files needed to build the package, excluding files listed in .gitignore or .cargo_vcs_info.json. <br/>    By default, the package is also built to verify it can be compiled successfully.<br/>    <br/>    Usually run without any additional arguments for single-package projects.
  - **Inputs:**
      - <code>all_features</code> : boolean<br />
      - <code>allow_dirty</code> : boolean<br />
      - <code>exclude</code> : string [ ]<br />
      - <code>exclude_lockfile</code> : boolean<br />
      - <code>features</code> : string [ ]<br />
      - <code>index</code> : string<br />
      - <code>jobs</code> : integer<br />
      - <code>keep_going</code> : boolean<br />
      - <code>list</code> : boolean<br />
      - <code>lockfile_path</code> : string<br />
      - <code>locking_mode</code> : string<br />
      - <code>manifest_path</code> : string<br />
      - <code>message_format</code> : string<br />
      - <code>no_default_features</code> : boolean<br />
      - <code>no_metadata</code> : boolean<br />
      - <code>no_verify</code> : boolean<br />
      - <code>output_verbosity</code> : string<br />
      - <code>package</code> : string [ ]<br />
      - <code>registry</code> : string<br />
      - <code>target</code> : string<br />
      - <code>target_dir</code> : string<br />
      - <code>toolchain</code> : string<br />
      - <code>workspace</code> : boolean<br />

- **cargo-remove**
  - Remove dependencies from a Cargo.toml manifest file.
  - **Inputs:**
      - <code>dep_id</code> : string [ ]<br />
      - <code>dependency_type</code> : string<br />
      - <code>dry_run</code> : boolean<br />
      - <code>lockfile_path</code> : string<br />
      - <code>locking_mode</code> : string<br />
      - <code>manifest_path</code> : string<br />
      - <code>output_verbosity</code> : string<br />
      - <code>package</code> : string<br />
      - <code>target</code> : string<br />
      - <code>toolchain</code> : string<br />

- **cargo-search**
  - Search packages in the registry. Default registry is crates.io. Equivalent to 'cargo search <code>QUERY</code>'.
  - **Inputs:**
      - <code>limit</code> : number<br />
      - <code>output_verbosity</code> : string<br />
      - <code>query</code> : string<br />
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
      - <code>ignore_rust_version</code> : boolean<br />
      - <code>jobs</code> : number<br />
      - <code>lib</code> : boolean<br />
      - <code>lockfile_path</code> : string<br />
      - <code>locking_mode</code> : string<br />
      - <code>manifest_path</code> : string<br />
      - <code>no_default_features</code> : boolean<br />
      - <code>no_fail_fast</code> : boolean<br />
      - <code>no_run</code> : boolean<br />
      - <code>output_verbosity</code> : string<br />
      - <code>package</code> : string<br />
      - <code>profile</code> : string<br />
      - <code>release</code> : boolean<br />
      - <code>target</code> : string<br />
      - <code>target_dir</code> : string<br />
      - <code>test</code> : string<br />
      - <code>test_args</code> : string [ ]<br />
      - <code>testname</code> : string<br />
      - <code>tests</code> : boolean<br />
      - <code>toolchain</code> : string<br />
      - <code>workspace</code> : boolean<br />

- **cargo-update**
  - Update dependencies as recorded in the local lock file. Updates the dependencies in Cargo.lock to their latest compatible versions.
  - **Inputs:**
      - <code>breaking</code> : boolean<br />
      - <code>config</code> : string<br />
      - <code>dry_run</code> : boolean<br />
      - <code>ignore_rust_version</code> : boolean<br />
      - <code>lockfile_path</code> : string<br />
      - <code>locking_mode</code> : string<br />
      - <code>manifest_path</code> : string<br />
      - <code>output_verbosity</code> : string<br />
      - <code>precise</code> : string<br />
      - <code>recursive</code> : boolean<br />
      - <code>spec</code> : string [ ]<br />
      - <code>toolchain</code> : string<br />
      - <code>unstable_flags</code> : string [ ]<br />
      - <code>workspace</code> : boolean<br />

- **rustc-explain**
  - Provide a detailed explanation of a Rust compiler error code. This tool allows AI agents to request more information about compilation errors by providing the error code (e.g., E0001, E0308, etc.). Very useful for understanding and resolving Rust compilation errors.
  - **Inputs:**
      - <code>error_code</code> : string<br />
      - <code>toolchain</code> : string<br />

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

  - URI: <a>cargo-book://CHANGELOG.md</a> <i>(text/markdown)</i>

- **Appendix: Git Authentication**

  - URI: <a>cargo-book://appendix/git-authentication.md</a> <i>(text/markdown)</i>

- **Appendix: Glossary**

  - URI: <a>cargo-book://appendix/glossary.md</a> <i>(text/markdown)</i>

- **Build Commands**

  - URI: <a>cargo-book://commands/build-commands.md</a> <i>(text/markdown)</i>

- **cargo add**

  - URI: <a>cargo-book://commands/cargo-add.md</a> <i>(text/markdown)</i>

- **cargo bench**

  - URI: <a>cargo-book://commands/cargo-bench.md</a> <i>(text/markdown)</i>

- **cargo build**

  - URI: <a>cargo-book://commands/cargo-build.md</a> <i>(text/markdown)</i>

- **cargo check**

  - URI: <a>cargo-book://commands/cargo-check.md</a> <i>(text/markdown)</i>

- **cargo clean**

  - URI: <a>cargo-book://commands/cargo-clean.md</a> <i>(text/markdown)</i>

- **cargo clippy**

  - URI: <a>cargo-book://commands/cargo-clippy.md</a> <i>(text/markdown)</i>

- **cargo doc**

  - URI: <a>cargo-book://commands/cargo-doc.md</a> <i>(text/markdown)</i>

- **cargo fetch**

  - URI: <a>cargo-book://commands/cargo-fetch.md</a> <i>(text/markdown)</i>

- **cargo fix**

  - URI: <a>cargo-book://commands/cargo-fix.md</a> <i>(text/markdown)</i>

- **cargo fmt**

  - URI: <a>cargo-book://commands/cargo-fmt.md</a> <i>(text/markdown)</i>

- **cargo generate-lockfile**

  - URI: <a>cargo-book://commands/cargo-generate-lockfile.md</a> <i>(text/markdown)</i>

- **cargo help**

  - URI: <a>cargo-book://commands/cargo-help.md</a> <i>(text/markdown)</i>

- **cargo info**

  - URI: <a>cargo-book://commands/cargo-info.md</a> <i>(text/markdown)</i>

- **cargo init**

  - URI: <a>cargo-book://commands/cargo-init.md</a> <i>(text/markdown)</i>

- **cargo install**

  - URI: <a>cargo-book://commands/cargo-install.md</a> <i>(text/markdown)</i>

- **cargo locate-project**

  - URI: <a>cargo-book://commands/cargo-locate-project.md</a> <i>(text/markdown)</i>

- **cargo login**

  - URI: <a>cargo-book://commands/cargo-login.md</a> <i>(text/markdown)</i>

- **cargo logout**

  - URI: <a>cargo-book://commands/cargo-logout.md</a> <i>(text/markdown)</i>

- **cargo metadata**

  - URI: <a>cargo-book://commands/cargo-metadata.md</a> <i>(text/markdown)</i>

- **cargo miri**

  - URI: <a>cargo-book://commands/cargo-miri.md</a> <i>(text/markdown)</i>

- **cargo new**

  - URI: <a>cargo-book://commands/cargo-new.md</a> <i>(text/markdown)</i>

- **cargo owner**

  - URI: <a>cargo-book://commands/cargo-owner.md</a> <i>(text/markdown)</i>

- **cargo package**

  - URI: <a>cargo-book://commands/cargo-package.md</a> <i>(text/markdown)</i>

- **cargo pkgid**

  - URI: <a>cargo-book://commands/cargo-pkgid.md</a> <i>(text/markdown)</i>

- **cargo publish**

  - URI: <a>cargo-book://commands/cargo-publish.md</a> <i>(text/markdown)</i>

- **cargo remove**

  - URI: <a>cargo-book://commands/cargo-remove.md</a> <i>(text/markdown)</i>

- **cargo report**

  - URI: <a>cargo-book://commands/cargo-report.md</a> <i>(text/markdown)</i>

- **cargo run**

  - URI: <a>cargo-book://commands/cargo-run.md</a> <i>(text/markdown)</i>

- **cargo rustc**

  - URI: <a>cargo-book://commands/cargo-rustc.md</a> <i>(text/markdown)</i>

- **cargo rustdoc**

  - URI: <a>cargo-book://commands/cargo-rustdoc.md</a> <i>(text/markdown)</i>

- **cargo search**

  - URI: <a>cargo-book://commands/cargo-search.md</a> <i>(text/markdown)</i>

- **cargo test**

  - URI: <a>cargo-book://commands/cargo-test.md</a> <i>(text/markdown)</i>

- **cargo tree**

  - URI: <a>cargo-book://commands/cargo-tree.md</a> <i>(text/markdown)</i>

- **cargo uninstall**

  - URI: <a>cargo-book://commands/cargo-uninstall.md</a> <i>(text/markdown)</i>

- **cargo update**

  - URI: <a>cargo-book://commands/cargo-update.md</a> <i>(text/markdown)</i>

- **cargo vendor**

  - URI: <a>cargo-book://commands/cargo-vendor.md</a> <i>(text/markdown)</i>

- **cargo version**

  - URI: <a>cargo-book://commands/cargo-version.md</a> <i>(text/markdown)</i>

- **cargo yank**

  - URI: <a>cargo-book://commands/cargo-yank.md</a> <i>(text/markdown)</i>

- **cargo**

  - URI: <a>cargo-book://commands/cargo.md</a> <i>(text/markdown)</i>

- **Deprecated and Removed Commands**

  - URI: <a>cargo-book://commands/deprecated-and-removed.md</a> <i>(text/markdown)</i>

- **General Commands**

  - URI: <a>cargo-book://commands/general-commands.md</a> <i>(text/markdown)</i>

- **Cargo Commands**

  - URI: <a>cargo-book://commands/index.md</a> <i>(text/markdown)</i>

- **Manifest Commands**

  - URI: <a>cargo-book://commands/manifest-commands.md</a> <i>(text/markdown)</i>

- **Package Commands**

  - URI: <a>cargo-book://commands/package-commands.md</a> <i>(text/markdown)</i>

- **Publishing Commands**

  - URI: <a>cargo-book://commands/publishing-commands.md</a> <i>(text/markdown)</i>

- **FAQ**

  - URI: <a>cargo-book://faq.md</a> <i>(text/markdown)</i>

- **First Steps with Cargo**

  - URI: <a>cargo-book://getting-started/first-steps.md</a> <i>(text/markdown)</i>

- **Getting Started with Cargo**

  - URI: <a>cargo-book://getting-started/index.md</a> <i>(text/markdown)</i>

- **Installing Cargo**

  - URI: <a>cargo-book://getting-started/installation.md</a> <i>(text/markdown)</i>

- **Cargo Home**

  - URI: <a>cargo-book://guide/cargo-home.md</a> <i>(text/markdown)</i>

- **Cargo.toml vs Cargo.lock**

  - URI: <a>cargo-book://guide/cargo-toml-vs-cargo-lock.md</a> <i>(text/markdown)</i>

- **Continuous Integration**

  - URI: <a>cargo-book://guide/continuous-integration.md</a> <i>(text/markdown)</i>

- **Creating a New Package**

  - URI: <a>cargo-book://guide/creating-a-new-project.md</a> <i>(text/markdown)</i>

- **Dependencies**

  - URI: <a>cargo-book://guide/dependencies.md</a> <i>(text/markdown)</i>

- **Cargo Guide**

  - URI: <a>cargo-book://guide/index.md</a> <i>(text/markdown)</i>

- **Package Layout**

  - URI: <a>cargo-book://guide/project-layout.md</a> <i>(text/markdown)</i>

- **Tests**

  - URI: <a>cargo-book://guide/tests.md</a> <i>(text/markdown)</i>

- **Why Cargo Exists**

  - URI: <a>cargo-book://guide/why-cargo-exists.md</a> <i>(text/markdown)</i>

- **Working on an Existing Package**

  - URI: <a>cargo-book://guide/working-on-an-existing-project.md</a> <i>(text/markdown)</i>

- **Introduction to Cargo**

  - URI: <a>cargo-book://index.md</a> <i>(text/markdown)</i>

- **Build Cache**

  - URI: <a>cargo-book://reference/build-cache.md</a> <i>(text/markdown)</i>

- **Build Script Examples**

  - URI: <a>cargo-book://reference/build-script-examples.md</a> <i>(text/markdown)</i>

- **Build Scripts**

  - URI: <a>cargo-book://reference/build-scripts.md</a> <i>(text/markdown)</i>

- **Cargo Targets**

  - URI: <a>cargo-book://reference/cargo-targets.md</a> <i>(text/markdown)</i>

- **Configuration**

  - URI: <a>cargo-book://reference/config.md</a> <i>(text/markdown)</i>

- **Credential Provider Protocol**

  - URI: <a>cargo-book://reference/credential-provider-protocol.md</a> <i>(text/markdown)</i>

- **Environment Variables**

  - URI: <a>cargo-book://reference/environment-variables.md</a> <i>(text/markdown)</i>

- **External Tools**

  - URI: <a>cargo-book://reference/external-tools.md</a> <i>(text/markdown)</i>

- **Features Examples**

  - URI: <a>cargo-book://reference/features-examples.md</a> <i>(text/markdown)</i>

- **Features**

  - URI: <a>cargo-book://reference/features.md</a> <i>(text/markdown)</i>

- **Future incompat report**

  - URI: <a>cargo-book://reference/future-incompat-report.md</a> <i>(text/markdown)</i>

- **Cargo Reference**

  - URI: <a>cargo-book://reference/index.md</a> <i>(text/markdown)</i>

- **Lints**

  - URI: <a>cargo-book://reference/lints.md</a> <i>(text/markdown)</i>

- **The Manifest Format**

  - URI: <a>cargo-book://reference/manifest.md</a> <i>(text/markdown)</i>

- **Overriding Dependencies**

  - URI: <a>cargo-book://reference/overriding-dependencies.md</a> <i>(text/markdown)</i>

- **Package ID Specifications**

  - URI: <a>cargo-book://reference/pkgid-spec.md</a> <i>(text/markdown)</i>

- **Profiles**

  - URI: <a>cargo-book://reference/profiles.md</a> <i>(text/markdown)</i>

- **Publishing on crates.io**

  - URI: <a>cargo-book://reference/publishing.md</a> <i>(text/markdown)</i>

- **Registries**

  - URI: <a>cargo-book://reference/registries.md</a> <i>(text/markdown)</i>

- **Registry Authentication**

  - URI: <a>cargo-book://reference/registry-authentication.md</a> <i>(text/markdown)</i>

- **Registry Index**

  - URI: <a>cargo-book://reference/registry-index.md</a> <i>(text/markdown)</i>

- **Registry Web API**

  - URI: <a>cargo-book://reference/registry-web-api.md</a> <i>(text/markdown)</i>

- **Dependency Resolution**

  - URI: <a>cargo-book://reference/resolver.md</a> <i>(text/markdown)</i>

- **Running a Registry**

  - URI: <a>cargo-book://reference/running-a-registry.md</a> <i>(text/markdown)</i>

- **Rust Version**

  - URI: <a>cargo-book://reference/rust-version.md</a> <i>(text/markdown)</i>

- **SemVer Compatibility**

  - URI: <a>cargo-book://reference/semver.md</a> <i>(text/markdown)</i>

- **Source Replacement**

  - URI: <a>cargo-book://reference/source-replacement.md</a> <i>(text/markdown)</i>

- **Specifying Dependencies**

  - URI: <a>cargo-book://reference/specifying-dependencies.md</a> <i>(text/markdown)</i>

- **Reporting build timings**

  - URI: <a>cargo-book://reference/timings.md</a> <i>(text/markdown)</i>

- **Unstable Features**

  - URI: <a>cargo-book://reference/unstable.md</a> <i>(text/markdown)</i>

- **Workspaces**

  - URI: <a>cargo-book://reference/workspaces.md</a> <i>(text/markdown)</i>


<sup>◾ generated by [mcp-discovery](https://github.com/rust-mcp-stack/mcp-discovery)</sup>
