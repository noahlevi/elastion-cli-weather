use clap::{Args, Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "weather")]
#[command(about = "CLI Weather forecast", long_about = None)]
pub struct WeatherArgs {
    #[clap(subcommand)]
    pub subcommand_: SubcommandType,
}

#[derive(Debug, Subcommand)]
pub enum SubcommandType {
    Configuration(ConfigurationArgs),
    Get(GetForecastArgs),
    Providers
}

#[derive(Debug, Args)]
pub struct ConfigurationArgs {
    pub provider: String,
}

#[derive(Debug, Args)]
pub struct GetForecastArgs {
    pub address: String,
    #[arg(help = "Correct date format is 'Y-m-d'")]
    pub date: Option<String>,
}