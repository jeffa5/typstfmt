use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::path::PathBuf;
use typstfmt::format;

use clap::Parser;
use clap::ValueEnum;

#[derive(Parser, Debug)]
#[command(version = "0.0.1", about = "A formatter for the typst language")]
struct Args {
    #[arg(
        short,
        long,
        value_enum, default_value_t = Mode::Format
    )]
    mode: Mode,

    /// A file to format. If not specified, all .typ file will be formatted
    #[arg()]
    input: Option<PathBuf>,

    /// If specified, the result of output will be put in this file. input *must* be specified if you set output.
    #[arg(short, long)]
    output: Option<PathBuf>,

    #[arg(long, default_value = "typstfmt.toml")]
    config: PathBuf,
}

#[derive(Copy, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Mode {
    /// formats the file in place
    Format,
    /// puts the formatted result in a __simulate__*.typ next to your inputs.
    Simulate,
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
        let mut file = File::options()
            .write(true)
            .append(false)
            .truncate(true)
            .open(&path)?;
        file.set_len(0)?;

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
            let mut file = File::open(output)?;
            file.write_all(formatted.as_bytes())?;
            break;
        }
        match args.mode {
            Mode::Format => {
                file.write_all(formatted.as_bytes())?;
            }
            Mode::Simulate => {
                let spath = path
                    .parent()
                    .unwrap_or(&PathBuf::default())
                    .join(path.file_stem().unwrap())
                    .join(&PathBuf::from("__simulate__.typ"));
                let mut file = File::create(&spath)?;
                file.write_all(formatted.as_bytes())?;
            }
        }
    }
    Ok(())
}
