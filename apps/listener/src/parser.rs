use crate::proto::MintEvent;
use crate::rpc::RawLog;
use std::{collections::HashMap, fs};

pub type LaunchpadMap = HashMap<String, String>;

pub fn load_map(path: &str) -> Result<LaunchpadMap, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(path)?;
    let map: LaunchpadMap = toml::from_str(&content)?;
    Ok(map)
}

pub fn parse(raw: &RawLog, map: &LaunchpadMap) -> Option<MintEvent> {
    if !raw
        .logs
        .iter()
        .any(|l| l.contains("Instruction: InitializeMint"))
    {
        return None;
    }
    let mint = extract_field(&raw.logs, "Mint \"")?;
    let authority = extract_field(&raw.logs, "Mint authority")?;
    let launchpad = map
        .get(&authority)
        .cloned()
        .unwrap_or_else(|| "Unknown".to_string());
    Some(MintEvent {
        mint_pubkey: mint,
        authority,
        program_id: raw.program_id.clone(),
        slot: raw.slot,
        launchpad,
    })
}

fn extract_field(logs: &[String], label: &str) -> Option<String> {
    for line in logs {
        if line.contains(label) {
            let pk = line.split('"').nth(1)?;
            return Some(pk.to_string());
        }
    }
    None
}
