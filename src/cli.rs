use clap::Parser;

#[derive(Debug, Parser)]
#[clap(version, about, long_about)]
pub struct Cli {
    /// Index of the source
    #[clap(short, long, group = "SOURCE", conflicts_with = "name")]
    pub index: Option<u32>,
    /// Name of the source
    #[clap(long, group = "SOURCE", conflicts_with = "index")]
    pub name: Option<String>,
    /// Text to print when muted
    #[clap(long, default_value = "muted")]
    pub muted: String,
    /// Text to print when unmuted
    #[clap(long, default_value = "unmuted")]
    pub unmuted: String,
}
