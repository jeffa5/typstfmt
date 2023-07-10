use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;
use tracing::debug;
use tracing::info;
use tracing::metadata::LevelFilter;
use tracing::warn;
use tracing_subscriber::EnvFilter;
use typstfmt::format;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version = "0.1.0", about = "Format typst code")]
struct Args {
    /// A file or directory to format. If not specified, all .typ files in the current directory will be formatted.
    files: Vec<PathBuf>,

    #[arg(long, default_value = "typstfmt.toml")]
    config_path: PathBuf,

    /// Run in 'check' mode. Exits with 0 if all input is formatted correctly. Exits with 1 if formatting of any input is required.
    #[arg(long)]
    check: bool,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::builder()
                .with_default_directive(LevelFilter::INFO.into())
                .from_env_lossy(),
        )
        .init();

    let paths: Vec<_> = if !args.files.is_empty() {
        args.files
            .iter()
            .flat_map(|f| {
                if f.is_file() {
                    vec![f.to_owned()]
                } else {
                    globmatch::Builder::new("**/*.typ")
                        .build(f)
                        .unwrap()
                        .into_iter()
                        .filter_map(|p| p.ok())
                        .collect()
                }
            })
            .collect()
    } else {
        let glob = globmatch::Builder::new("**/*.typ").build(".").unwrap();
        glob.into_iter().flat_map(|path| path.ok()).collect()
    };

    let mut check_ok = true;
    for path in paths.into_iter() {
        match format_file(&path, &args) {
            Ok(()) => {
                info!(?path, "Successfully formatted file");
            }
            Err(error) => {
                warn!(?path, ?error, "Failed to format file");
                check_ok = false;
            }
        }
    }

    if args.check && !check_ok {
        anyhow::bail!("Failed check");
    }

    Ok(())
}

fn format_file(path: &Path, args: &Args) -> anyhow::Result<()> {
    let mut file = File::options()
        .read(true)
        .open(&path)
        .unwrap_or_else(|e| panic!("Couldn't open file : {e}"));
    let mut content = String::with_capacity(1024);
    file.read_to_string(&mut content)?;
    drop(file);

    let config = if args.config_path.is_file() {
        let mut config_file = File::open(&args.config_path)?;
        let mut config_file_content = String::new();
        config_file.read_to_string(&mut config_file_content)?;
        toml::from_str(&config_file_content)?
    } else {
        typstfmt::Config::default()
    };

    debug!(?path, "Formatting input");

    let formatted = format(&content, config)?;
    if args.check {
        if formatted != content {
            anyhow::bail!("Output still needs formatting");
        }
    } else {
        let mut file = File::create(&path)?;
        file.write_all(formatted.as_bytes())?;
    }
    Ok(())
}
