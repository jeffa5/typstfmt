use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::path::PathBuf;
use typstfmt::format;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version = "0.0.1", about = "A formatter for the typst language")]
struct Args {
    /// A file to format. If not specified, all .typ file will be formatted
    #[arg()]
    input: Option<PathBuf>,

    /// If specified, the result of output will be put in this file. input *must* be specified if you set output.
    #[arg(short, long)]
    output: Option<PathBuf>,

    #[arg(long, default_value = "typstfmt.toml")]
    config: PathBuf,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    if args.output.is_some() && args.input.is_none() {
        panic!("Input must be specified to use an output.")
    }
    let paths = if let Some(input) = args.input {
        vec![input]
    } else {
        let glob = globmatch::Builder::new("**.typ").build(".").unwrap();
        glob.into_iter().flat_map(|path| path.ok()).collect()
    };
    for path in paths.into_iter() {
        let mut file = File::options()
            .read(true)
            .open(&path)
            .unwrap_or_else(|e| panic!("Couldn't open input file : {e}"));
        let mut content = String::with_capacity(1024);
        file.read_to_string(&mut content)?;
        drop(file);

        let config = if args.config.is_file() {
            let mut config_file = File::open(&args.config)?;
            let mut config_file_content = String::new();
            config_file.read_to_string(&mut config_file_content)?;
            toml::from_str(&config_file_content)?
        } else {
            typstfmt::Config::default()
        };

        let formatted = format(&content, config)?;
        if let Some(output) = args.output {
            let mut file = File::create(output)?;
            file.write_all(formatted.as_bytes())?;
            break;
        }

        let mut file = File::create(&path)?;
        file.write_all(formatted.as_bytes())?;
    }
    Ok(())
}
