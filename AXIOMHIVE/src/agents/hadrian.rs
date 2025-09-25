// @AXIOMHIVE @DEVDOLLZAI
// Hadrian: Orchestrates Dagger agents to construct competitive moats.

use crate::agents::cerebrum::Strategy;
use crate::compiler::Compiler;
use std::path::PathBuf;
use anyhow::Result;

pub struct Hadrian;

impl Hadrian {
    pub fn new() -> Self {
        Hadrian
    }

    pub fn orchestrate_and_compile(&self, strategy: &Strategy) -> Result<PathBuf> {
        let core_task = &strategy.core_task;

        println!("HADRIAN :: Orchestrating Compiler for task: {}", core_task);

        match core_task.as_str() {
            "GENERATE_DATA_LOOP_ARCHITECTURE" => {
                Compiler::compile_data_moat_service()
            }
            "GENERATE_SIMPLIFICATION_WORKFLOW" => {
                // For now, same as data moat, can extend later
                Compiler::compile_data_moat_service()
            }
            _ => {
                Err(anyhow::anyhow!("Unrecognized strategic task: {}", core_task))
            }
        }
    }
}