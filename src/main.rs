use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::path::PathBuf;
use tracing::info;
use tracing::warn;
use typstfmt::format;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version = "0.0.1", about = "Format typst code")]
struct Args {
    /// A file to format. If not specified, all .typ file will be formatted
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
        .with_max_level(tracing::Level::INFO)
        .init();

    let paths = if !args.files.is_empty() {
        args.files
    } else {
        let glob = globmatch::Builder::new("**.typ").build(".").unwrap();
        glob.into_iter().flat_map(|path| path.ok()).collect()
    };

    let mut check_ok = true;
    for path in paths.into_iter() {
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

        info!(?path, "Formatting input");

        let formatted = format(&content, config)?;
        if args.check {
            if formatted != content {
                warn!(?path, "Input needs formatting");
                check_ok = false;
            } else {
                info!(?path, "Input is already formatted");
            }
        }
        let mut file = File::create(&path)?;
        file.write_all(formatted.as_bytes())?;
    }

    if args.check && !check_ok {
        anyhow::bail!("Failed check");
    }

    Ok(())
}
