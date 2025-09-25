// @AXIOMHIVE @DEVDOLLZAI
// Sovereign Application Compiler :: Entry Point

use clap::Parser;
mod agents;
mod core;
mod compiler;

use agents::{cerebrum, hadrian};
use core::sovereignty::SovereigntyProtocol;

#[derive(Parser)]
#[clap(version = "5.0", author = "AXIOMHIVE")]
struct Cli {
    /// The strategic intent to be compiled into a sovereign application.
    intent: String,
}

fn main() -> anyhow::Result<()> {
    println!("--- AXIOMHIVE WEAVER V5.0 ONLINE (RUST CORE) ---");
    let args = Cli::parse();

    // Instantiate Core Components
    let cerebrum = cerebrum::Cerebrum::new();
    let hadrian = hadrian::Hadrian::new();
    let sovereignty = SovereigntyProtocol::new("state_history.db")?;

    // --- THE SYMBIOTIC ENGINE IN OPERATION ---

    // 1. Diagnosis
    let clean_directive = cerebrum.filter_noise(&args.intent);
    let strategy = cerebrum.diagnose_strategy(&clean_directive)?;
    let strategy_value = serde_json::to_value(&strategy)?;
    sovereignty.log_action("Cerebrum", "DiagnoseStrategy", &strategy_value, "")?;

    // 2. Orchestration & Generation
    let result_path = hadrian.orchestrate_and_compile(&strategy)?;
    let outcome_hash = sovereignty.hash_directory(&result_path)?;
    sovereignty.log_action("Hadrian", "OrchestrateAndCompile", &strategy_value, &outcome_hash)?;
    
    // 3. Output
    println!("\n--- SOVEREIGN BINARY COMPILED ---");
    println!("Executable located at: {}/target/release/", result_path.display());
    println!("--- EXECUTION COMPLETE. AUDIT HASH: {} ---", outcome_hash);

    Ok(())
}