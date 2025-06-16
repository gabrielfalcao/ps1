#![allow(unused)]
use std::collections::BTreeMap;

use iocore::Path;
use serde::{Deserialize, Serialize};

use crate::{parse_tokens, Definition, Error, Result, Rule};

#[derive(
    Clone, PartialOrd, Ord, PartialEq, Eq, Hash, Deserialize, Serialize, Debug,
)]
pub struct Config {
    pub ps1: String,
    pub vcs: Option<VcsConfig>,
    pub variants: BTreeMap<String, String>,
    pub enable_colors: Option<bool>,
}
impl Config {
    pub fn ps1(&self) -> String {
        let ps1 = self.ps1.clone();
        // let ps1 = iocore::env::var("PS1_VARIANT")
        //     .map(|variant| {
        //         self.variants.get(&variant).map(Clone::clone).unwrap_or_else(|| ps1.clone())
        //     })
        //     .unwrap_or_else(|_| ps1.clone())
        //     .to_string();
        ps1
    }

    pub fn render_ps1(&self) -> Result<String> {
        let tokens = parse_tokens(&self.ps1())?;
        let ps1 = tokens
            .iter()
            .map(|token| token.to_string())
            .collect::<String>();
        // let user = User::id().unwrap_or_default();
        // ps1 = ps1.replace(r"\u", user.name.as_str());
        // // let (_, hostname, _) = iocore::shell_command_string_output("hostname", ".")?;
        // // ps1 = ps1.replace(r"\h", hostname.as_str());
        // let cwd = Path::cwd().try_canonicalize();
        // if cwd == user.home().unwrap().into() {
        //     ps1 = ps1.replace(r"\w", "~");
        //     ps1 = ps1.replace(r"\W", "~");
        // } else {
        //     ps1 = ps1.replace(r"\w", cwd.to_string().as_str());
        //     ps1 = ps1.replace(r"\W", cwd.name().to_string().as_str());
        // }
        Ok(ps1)
    }

    pub fn new(ps1: &str) -> Config {
        Config::with_vcs(ps1, VcsConfig::default())
    }

    pub fn with_vcs(ps1: &str, vcs: VcsConfig) -> Config {
        Config::with_options(
            ps1,
            vcs,
            console::Term::stdout()
                .features()
                .colors_supported(),
            Vec::new(),
        )
    }

    pub fn with_options(
        ps1: &str,
        vcs: VcsConfig,
        enable_colors: bool,
        variants: Vec<(String, String)>,
    ) -> Config {
        Config {
            ps1: ps1.to_string(),
            vcs: Some(vcs),
            variants: BTreeMap::from_iter(variants.iter().map(Clone::clone)),
            enable_colors: Some(enable_colors),
        }
    }
}

impl Default for Config {
    fn default() -> Config {
        Config {
            ps1: r"{220}\u{231}:{220}\W{37}{git:branch}{220}\${reset}".to_string(),
            enable_colors: None,
            vcs: None,
            variants: Default::default(),
        }
    }
}
#[derive(
    Clone, PartialOrd, Ord, PartialEq, Eq, Hash, Deserialize, Serialize, Debug, Default,
)]
pub struct VcsConfig {
    pub git: Option<String>,
    pub mercurial: Option<String>,
}
impl Config {
    pub fn load(path: &Path) -> Result<Config> {
        Ok(Config::from_toml_string(&path.read()?)?)
    }

    pub fn from_toml_string(config: &str) -> Result<Config> {
        Ok(toml::from_str::<Config>(config)?)
    }
}
