//! VerityPay conformance suite CLI.

use std::path::PathBuf;
use std::process;

use clap::{Parser, Subcommand, ValueEnum};
use vp_conformance_cli::{
    exit_code_from_report, parse_adapter_outcome, run_scenario, RunError, RunOptions,
};
use vp_conformance_cli::{EXIT_HARNESS_ERROR, EXIT_USER_ERROR};
use vp_reference_model::Outcome;

#[derive(Debug, Parser)]
#[command(name = "vp-conformance")]
#[command(about = "VerityPay conformance suite")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Run one VP-CS scenario through the conformance pipeline.
    Run(RunArgs),
}

#[derive(Debug, clap::Args)]
struct RunArgs {
    /// Path to a VP-CS scenario fixture file.
    #[arg(long)]
    scenario: PathBuf,

    /// Implementation adapter to invoke.
    #[arg(long, default_value = "stub")]
    adapter: String,

    /// Outcome returned by the stub adapter.
    #[arg(long, default_value = "satisfied", value_parser = parse_adapter_outcome)]
    adapter_outcome: Outcome,

    /// Report output format.
    #[arg(long, default_value = "human", value_enum)]
    format: ReportFormat,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
enum ReportFormat {
    Human,
    Json,
}

impl From<ReportFormat> for vp_conformance_cli::OutputFormat {
    fn from(format: ReportFormat) -> Self {
        match format {
            ReportFormat::Human => Self::Human,
            ReportFormat::Json => Self::Json,
        }
    }
}

fn main() {
    let cli = Cli::parse();

    let exit_code = match cli.command {
        Commands::Run(args) => match execute_run(args) {
            Ok(code) => code,
            Err(error) => {
                eprintln!("error: {error}");
                match error {
                    RunError::User(_) => EXIT_USER_ERROR,
                    RunError::Harness(_) => EXIT_HARNESS_ERROR,
                }
            }
        },
    };

    process::exit(exit_code);
}

fn execute_run(args: RunArgs) -> Result<i32, RunError> {
    let options = RunOptions::new(
        args.scenario,
        args.adapter,
        args.adapter_outcome,
        args.format.into(),
    );
    let output = run_scenario(&options)?;

    print!("{}", output.rendered());

    Ok(exit_code_from_report(output.report()))
}
