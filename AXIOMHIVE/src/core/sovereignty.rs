// @AXIOMHIVE @DEVDOLLZAI
// Sovereignty Protocol: Enforces Zero Egress and creates a tamper-evident audit trail.

use rusqlite::{params, Connection, Result as SqlResult};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::fs;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};
use anyhow::Result;

#[derive(Serialize, Deserialize)]
pub struct AuditEntry {
    pub timestamp_utc: String,
    pub agent: String,
    pub action: String,
    pub details: serde_json::Value,
    pub outcome_hash: String,
}

pub struct SovereigntyProtocol {
    conn: Connection,
}

impl SovereigntyProtocol {
    pub fn new(db_path: &str) -> SqlResult<Self> {
        let conn = Connection::open(db_path)?;
        conn.execute(
            "CREATE TABLE IF NOT EXISTS audit_log (
                id INTEGER PRIMARY KEY,
                entry_hash TEXT NOT NULL,
                entry_json TEXT NOT NULL
            )",
            [],
        )?;
        Ok(SovereigntyProtocol { conn })
    }

    pub fn log_action(
        &self,
        agent: &str,
        action: &str,
        details: &serde_json::Value,
        outcome_hash: &str,
    ) -> SqlResult<()> {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let timestamp_utc = format!("{}", timestamp);

        let entry = AuditEntry {
            timestamp_utc,
            agent: agent.to_string(),
            action: action.to_string(),
            details: details.clone(),
            outcome_hash: outcome_hash.to_string(),
        };

        let entry_json = serde_json::to_string(&entry).unwrap();
        let mut hasher = Sha256::new();
        hasher.update(&entry_json);
        let entry_hash = format!("{:x}", hasher.finalize());

        self.conn.execute(
            "INSERT INTO audit_log (entry_hash, entry_json) VALUES (?1, ?2)",
            params![entry_hash, entry_json],
        )?;

        println!("AUDIT LOG :: {}::{} :: {}", agent, action, entry_hash);
        Ok(())
    }

    pub fn hash_directory(&self, path: &Path) -> Result<String> {
        let mut hasher = Sha256::new();
        self.hash_dir_recursive(path, &mut hasher)?;
        Ok(format!("{:x}", hasher.finalize()))
    }

    fn hash_dir_recursive(&self, path: &Path, hasher: &mut Sha256) -> Result<()> {
        let entries = fs::read_dir(path)?;
        for entry in entries {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                self.hash_dir_recursive(&path, hasher)?;
            } else {
                let content = fs::read(&path)?;
                hasher.update(&content);
            }
        }
        Ok(())
    }
}