use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::path::PathBuf;
use typstfmt::format;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version = "0.0.1", about = "Format typst code")]
struct Args {
    /// A file to format. If not specified, all .typ file will be formatted
    files: Vec<PathBuf>,

    #[arg(long, default_value = "typstfmt.toml")]
    config_path: PathBuf,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let paths = if !args.files.is_empty() {
        args.files
    } else {
        let glob = globmatch::Builder::new("**.typ").build(".").unwrap();
        glob.into_iter().flat_map(|path| path.ok()).collect()
    };
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

        let formatted = format(&content, config)?;
        let mut file = File::create(&path)?;
        file.write_all(formatted.as_bytes())?;

        let mut file = File::create(&path)?;
        file.write_all(formatted.as_bytes())?;
    }
    Ok(())
}
