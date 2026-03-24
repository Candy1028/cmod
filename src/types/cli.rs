use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "cmod", version = "1.11", about = "交互式 Go 包检索与安装工具")]
pub struct Cli {
    #[arg(required_unless_present = "old", conflicts_with = "old")]
    pub target: Option<String>,
    #[arg(short, long, default_value_t = 25, conflicts_with = "old")]
    pub limit: u64,
    #[arg(short,long, help = "Print Installed Packages")]
    pub old: bool,
}
impl Cli{
    pub fn new() -> Cli{
        Cli::parse()
    }
}