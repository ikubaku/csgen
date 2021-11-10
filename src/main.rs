use std::fs::File;
use std::io::{Read, Write};
use std::path::PathBuf;
use std::str::FromStr;
use structopt::StructOpt;
use crate::csgen::CsGen;

use crate::csgen::settings::GenerationSettings;

mod csgen;


#[derive(StructOpt)]
#[structopt(name = "csgen")]
struct Opt {
    #[structopt(short, long)]
    create_default: bool,

    #[structopt(short, long, parse(from_os_str))]
    settings_filename: Option<PathBuf>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opt: Opt = Opt::from_args();

    if opt.create_default {
        let mut template_file = File::create(PathBuf::from_str("csgen_settings.yaml")?)?;
        let default_settings = GenerationSettings::default();
        template_file.write_fmt(format_args!("{}", serde_yaml::to_string(&default_settings)?.as_str()))?;
        return Ok(());
    }

    let settings_filename =
        opt.settings_filename.ok_or(
            std::io::Error::new(std::io::ErrorKind::NotFound, "Expected settings filename."))?;

    let settings: GenerationSettings = {
        let mut settings_file = File::open(settings_filename)?;
        let mut settings_content = String::new();
        settings_file.read_to_string(&mut settings_content)?;
        serde_yaml::from_str(settings_content.as_str())?
    };

    let generator = CsGen::from_settings(settings);
    generator.generate();

    Ok(())
}
