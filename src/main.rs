use argh::FromArgs;
use std::{env, fs, path::PathBuf};

#[derive(FromArgs, Debug)]
/// Cross platform file copy tool
struct Options {
    /// source glob
    #[argh(positional)]
    source_glob: String,

    /// destination directory
    #[argh(positional, from_str_fn(parse_path))]
    destination_dir: PathBuf,

    /// preserve directory structure in the output directory
    #[argh(switch, short = 'p')]
    preserve_directories: bool,
}

fn parse_path(value: &str) -> Result<PathBuf, String> {
    Ok(PathBuf::from(value))
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let options: Options = argh::from_env();

    fs::create_dir_all(&options.destination_dir)?;

    for file in globwalk::glob(options.source_glob)? {
        if let Ok(src_file) = file {
            let dst_file = if options.preserve_directories {
                let abs = src_file.path().canonicalize()?;
                let abs = abs.strip_prefix(env::current_dir()?.canonicalize()?)?;
                options.destination_dir.join(abs)
            } else {
                options.destination_dir.join(src_file.file_name())
            };

            fs::create_dir_all(&dst_file.parent().expect("Does not have a parent directory"))?;

            fs::copy(src_file.path(), dst_file)?;
        }
    }

    Ok(())
}
