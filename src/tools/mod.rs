pub mod cargo;
pub mod cargo_deny;
pub mod cargo_hack;
pub mod cargo_machete;
pub mod rustc;
pub mod rustup;

use cargo::{
    CargoAddRmcpTool, CargoBuildRmcpTool, CargoCheckRmcpTool, CargoCleanRmcpTool,
    CargoClippyRmcpTool, CargoDocRmcpTool, CargoFmtRmcpTool, CargoGenerateLockfileRmcpTool,
    CargoInfoRmcpTool, CargoListRmcpTool, CargoMetadataRmcpTool, CargoNewRmcpTool,
    CargoPackageRmcpTool, CargoRemoveRmcpTool, CargoSearchRmcpTool, CargoTestRmcpTool,
    CargoUpdateRmcpTool,
};
use cargo_deny::{
    CargoDenyCheckRmcpTool, CargoDenyInitRmcpTool, CargoDenyInstallRmcpTool, CargoDenyListRmcpTool,
};
use cargo_hack::{CargoHackInstallRmcpTool, CargoHackRmcpTool};
use cargo_machete::{CargoMacheteInstallRmcpTool, CargoMacheteRmcpTool};
use rustc::RustcExplainRmcpTool;
use rustup::{RustupShowRmcpTool, RustupToolchainAddRmcpTool, RustupUpdateRmcpTool};

static WORKSPACE_ROOT: std::sync::OnceLock<String> = std::sync::OnceLock::new();

pub fn set_workspace_root(root: String) {
    WORKSPACE_ROOT
        .set(root)
        .expect("Workspace root can only be set once");
}

pub fn apply_workspace_root(cmd: &mut std::process::Command) {
    if let Some(root) = WORKSPACE_ROOT.get() {
        cmd.current_dir(root);
    }
}
