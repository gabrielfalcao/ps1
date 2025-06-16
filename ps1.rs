use ps1g::{Config, Result, resolve};
use clap::Parser;
use iocore::Path;

#[derive(Parser, Debug)]
#[command(
    author,
    version,
    about,
    long_about = "ps1 command-line utility"
)]
pub struct Cli {
    #[arg(
        short,
        long,
        env = "PS1_CONFIG_PATH"
    )]
    config_path: Option<Path>,

    #[arg(long)]
    env: bool,

    #[arg(short, long)]
    var: bool,

    #[arg(
        short,
        long,
        conflicts_with_all = ["env", "var"]
    )]
    resolve: Option<String>,
}
impl Cli {
    pub fn config_path(&self) -> Path {
        match &self.config_path {
            Some(path) => path.clone(),
            None => Path::raw("~/.config/ps1.toml").try_canonicalize(),
        }
    }

    pub fn config(&self) -> Config {
        Config::load(&self.config_path()).unwrap_or_default()
    }
}

fn main() -> Result<()> {
    let opts = Cli::parse();
    let config = opts.config();
    let rendered = config.render_ps1()?;
    if opts.env || opts.var {
        println!(
            "{}",
            [
                format!("export PS1='{}';", &rendered),
                format!("unset PROMPT_COMMAND;")
            ]
            .join("\n")
        )
    } else if let Some(resolve) = opts.resolve.clone() {
        print!(
            "{}",
            match resolve.as_str() {
                "cwd:name" => {
                    resolve::cwd_name()
                },
                "git:branch" => {
                    resolve::git_branch()
                },
                val => {
                    val.to_string()
                },
            }
        );
    }
    Ok(())
}
