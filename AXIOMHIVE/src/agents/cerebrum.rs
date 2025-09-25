// @AXIOMHIVE @DEVDOLLZAI
// Cerebrum: Deconstructs intent and maps it to strategic objectives.

use serde::{Deserialize, Serialize};
use anyhow::Result;

#[derive(Serialize, Deserialize, Debug)]
pub struct Strategy {
    pub primary_moat: String,
    pub complexity_type: String,
    pub core_task: String,
}

pub struct Cerebrum {
    noise_words: Vec<String>,
}

impl Cerebrum {
    pub fn new() -> Self {
        Cerebrum {
            noise_words: vec![
                "please".to_string(),
                "can you".to_string(),
                "design".to_string(),
                "a".to_string(),
                "an".to_string(),
                "the".to_string(),
                "for".to_string(),
                "of".to_string(),
                "to".to_string(),
            ],
        }
    }

    pub fn filter_noise(&self, directive: &str) -> String {
        let tokens: Vec<String> = directive
            .to_lowercase()
            .split_whitespace()
            .map(|s| s.to_string())
            .collect();
        let filtered: Vec<String> = tokens
            .into_iter()
            .filter(|t| !self.noise_words.contains(t))
            .collect();
        filtered.join(" ")
    }

    pub fn diagnose_strategy(&self, clean_directive: &str) -> Result<Strategy> {
        let mut strategy = Strategy {
            primary_moat: "None".to_string(),
            complexity_type: "None".to_string(),
            core_task: "Unknown".to_string(),
        };

        if clean_directive.contains("data moat") || clean_directive.contains("feedback loop") {
            strategy.primary_moat = "Data".to_string();
            strategy.core_task = "GENERATE_DATA_LOOP_ARCHITECTURE".to_string();
        } else if clean_directive.contains("simplify") || clean_directive.contains("reduce complexity") {
            strategy.primary_moat = "Operational".to_string();
            strategy.core_task = "GENERATE_SIMPLIFICATION_WORKFLOW".to_string();
        }

        if clean_directive.contains("architecture") {
            strategy.complexity_type = "System/Technical".to_string();
        } else if clean_directive.contains("process") {
            strategy.complexity_type = "Process".to_string();
        }

        println!("CEREBRUM :: Intent diagnosed. Moat Target={}.", strategy.primary_moat);
        Ok(strategy)
    }
}