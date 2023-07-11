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
use typstfmt::FormatError;

use clap::Parser;

#[derive(Debug, thiserror::Error)]
enum Error {
    #[error("format error: {0}")]
    FormatError(#[from] FormatError),
    #[error("check failed")]
    CheckFailed,
    #[error("io error: {0}")]
    IOError(#[from] std::io::Error),
    #[error("toml config error: {0}")]
    TomlError(#[from] toml::de::Error),
}

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

    let mut ok = 0;
    let mut erroneous = 0;
    let mut needs_formatting = 0;
    for path in paths.into_iter() {
        match format_file(&path, &args) {
            Ok(()) => {
                info!(?path, "Successfully formatted file");
                ok += 1;
            }
            Err(error) => {
                match error {
                    Error::FormatError(_) => {
                        warn!(?path, %error, "Failed to format file");
                        erroneous += 1;
                    }
                    Error::CheckFailed => {
                        warn!(?path, "Failed check");
                        needs_formatting += 1;
                    }
                    Error::IOError(_) => {
                        warn!(?path, %error, "Got an error")
                    }
                    Error::TomlError(_) => {
                        warn!(?path, %error, "Failed to get config")
                    }
                }
            }
        }
    }

    if args.check {
        if needs_formatting > 0 {
            anyhow::bail!(
                "Failed check. {} files passed, {} had errors, {} need formatting",
                ok,
                erroneous,
                needs_formatting
            );
        } else {
            println!(
                "Passed check. {} files passed, {} had errors",
                ok, erroneous
            );
        }
    } else {
        println!("Formatted {} files. {} files had errors", ok, erroneous);
    }

    Ok(())
}

fn format_file(path: &Path, args: &Args) -> Result<(), Error> {
    let mut file = File::options()
        .read(true)
        .open(path)
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
            return Err(Error::CheckFailed);
        }
    } else {
        let mut file = File::create(path)?;
        file.write_all(formatted.as_bytes())?;
    }
    Ok(())
}
