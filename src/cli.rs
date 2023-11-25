use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    pub dir: Option<String>,
    #[arg(short, long)]
    pub spacer: Option<String>,
    #[arg(short, long)]
    pub include_hidden: bool,
    #[arg(short, long)]
    pub git_ignored: bool,
    #[arg(short, long)]
    pub copy: bool,
    #[arg(short, long)]
    pub prefix: Option<String>,
}
