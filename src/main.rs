use std::fs::File;
use std::io::stdin;
use std::io::Read;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;
use std::time::Instant;
use tracing::debug;
use tracing::info;
use tracing::metadata::LevelFilter;
use tracing::warn;
use tracing_subscriber::EnvFilter;
use typstfmt::format;
use typstfmt::Config;
use typstfmt::FormatError;

use clap::Parser;

#[derive(Debug, thiserror::Error)]
enum Error {
    #[error("format error: {0}")]
    Format(#[from] FormatError),
    #[error("check failed")]
    CheckFailed,
    #[error("io error: {0}")]
    IO(#[from] std::io::Error),
    #[error("toml config error: {0}")]
    Toml(#[from] toml::de::Error),
}

#[derive(Parser, Debug)]
#[command(version = "0.1.0", about = "Format typst code")]
struct Args {
    /// A file or directory to format. If not specified, stdin is read for input.
    files: Vec<PathBuf>,

    #[arg(long, default_value = "typstfmt.toml")]
    config_path: PathBuf,

    /// Run in 'check' mode. Exits with 0 if all input is formatted correctly. Exits with 1 if formatting of any input is required.
    #[arg(long)]
    check: bool,

    /// Print out the diff between the original content and the formatted content.
    /// Also behaves like 'check' mode for exit codes.
    #[arg(long)]
    diff: bool,
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
        Vec::new()
    };

    let config = if args.config_path.is_file() {
        debug!(config_path=?args.config_path, "Loading config from file");
        let mut config_file = File::open(&args.config_path)?;
        let mut config_file_content = String::new();
        config_file.read_to_string(&mut config_file_content)?;
        toml::from_str(&config_file_content)?
    } else {
        debug!("Using default config");
        typstfmt::Config::default()
    };

    let start = Instant::now();

    let mut formatted = 0;
    let mut unchanged = 0;
    let mut erroneous = 0;
    if paths.is_empty() {
        // read from stdin
        let input = std::io::read_to_string(stdin())?;
        match format_stdin(&input, &config, &args) {
            Ok(did_format) => match did_format {
                DidFormat::Yes => {
                    info!("Successfully formatted stdin");
                    formatted += 1;
                }
                DidFormat::No => {
                    info!("Already correctly formatted");
                    unchanged += 1;
                }
            },
            Err(error) => match error {
                Error::Format(_) => {
                    warn!(%error, "Failed to format stdin");
                    erroneous += 1;
                }
                Error::CheckFailed => {
                    warn!("Failed check");
                    formatted += 1;
                }
                Error::IO(_) => {
                    warn!(%error, "Got an error")
                }
                Error::Toml(_) => {
                    warn!(%error, "Failed to get config")
                }
            },
        }
    } else {
        for path in paths.into_iter() {
            match format_file(&path, &config, &args) {
                Ok(did_format) => match did_format {
                    DidFormat::Yes => {
                        info!(?path, "Successfully formatted file");
                        formatted += 1;
                    }
                    DidFormat::No => {
                        info!(?path, "Already correctly formatted");
                        unchanged += 1;
                    }
                },
                Err(error) => match error {
                    Error::Format(_) => {
                        warn!(?path, %error, "Failed to format file");
                        erroneous += 1;
                    }
                    Error::CheckFailed => {
                        warn!(?path, "Failed check");
                        formatted += 1;
                    }
                    Error::IO(_) => {
                        warn!(?path, %error, "Got an error")
                    }
                    Error::Toml(_) => {
                        warn!(?path, %error, "Failed to get config")
                    }
                },
            }
        }
    }

    let elapsed = start.elapsed();

    if args.check {
        if formatted > 0 {
            anyhow::bail!(
                "Failed check. {} files passed, {} had errors, {} need formatting.",
                unchanged,
                erroneous,
                formatted
            );
        } else {
            println!(
                "Passed check. {} files passed, {} had errors, {:?}.",
                unchanged, erroneous, elapsed
            );
        }
    } else {
        println!(
            "{} files formatted. {} files were already correct. {} files had errors, {:?}.",
            formatted, unchanged, erroneous, elapsed
        );
    }

    Ok(())
}

enum DidFormat {
    Yes,
    No,
}

fn format_file(path: &Path, config: &Config, args: &Args) -> Result<DidFormat, Error> {
    let mut file = File::options()
        .read(true)
        .open(path)
        .unwrap_or_else(|e| panic!("Couldn't open file : {e}"));
    let mut content = String::with_capacity(1024);
    file.read_to_string(&mut content)?;
    drop(file);

    debug!(?path, "Formatting input");

    // TODO: remove this clone, format should take a &Config
    let formatted = format(&content, config.clone())?;

    let did_format = if formatted == content {
        DidFormat::No
    } else {
        DidFormat::Yes
    };

    if args.diff {
        let text_diff = similar::TextDiff::from_lines(&content, &formatted);
        println!(
            "{}",
            text_diff.unified_diff().header(
                path.to_str().unwrap(),
                &format!("{}.formatted", path.to_str().unwrap())
            )
        );
        if matches!(did_format, DidFormat::Yes) {
            return Err(Error::CheckFailed);
        } else {
            return Ok(did_format);
        }
    }

    if args.check {
        if matches!(did_format, DidFormat::Yes) {
            return Err(Error::CheckFailed);
        }
    } else if matches!(did_format, DidFormat::Yes) {
        let mut file = File::create(path)?;
        file.write_all(formatted.as_bytes())?;
    }
    Ok(did_format)
}

fn format_stdin(content: &str, config: &Config, args: &Args) -> Result<DidFormat, Error> {
    debug!("Formatting stdin");

    // TODO: remove this clone, format should take a &Config
    let formatted = format(content, config.clone())?;

    let did_format = if formatted == content {
        DidFormat::No
    } else {
        DidFormat::Yes
    };

    if args.diff {
        let text_diff = similar::TextDiff::from_lines(content, &formatted);
        println!(
            "{}",
            text_diff
                .unified_diff()
                .header("stdin", &format!("{}.formatted", "stdin"))
        );
        if matches!(did_format, DidFormat::Yes) {
            return Err(Error::CheckFailed);
        } else {
            return Ok(did_format);
        }
    }

    if args.check {
        if matches!(did_format, DidFormat::Yes) {
            return Err(Error::CheckFailed);
        }
    } else if matches!(did_format, DidFormat::Yes) {
        println!("{}", formatted);
    }
    Ok(did_format)
}
