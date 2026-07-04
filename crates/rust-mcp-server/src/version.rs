const VERSION: &str = env!("CARGO_PKG_VERSION");
const GIT_HASH: Option<&str> = option_env!("GIT_HASH");

pub struct AppVersion;

impl AppVersion {
    pub fn version() -> String {
        match GIT_HASH {
            Some(hash) => format!("{VERSION}.{hash}"),
            None => VERSION.into(),
        }
    }
}

impl From<AppVersion> for clap::builder::Str {
    fn from(_: AppVersion) -> Self {
        AppVersion::version().into()
    }
}
