use std::path::PathBuf;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PyrightOutput {
    pub version: String,
    pub time: String,
    pub general_diagnostics: Vec<Diagnostic>,
    pub summary: Summary,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Diagnostic {
    pub file: PathBuf,
    pub severity: Severity,
    pub rule: Option<String>,
    pub message: String,
    pub range: Range,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Range {
    pub start: Position,
    pub end: Position,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Position {
    pub line: u64,
    pub character: u64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Summary {
    pub files_analyzed: i64,
    pub time_in_sec: f64,
    pub error_count: i64,
    pub warning_count: i64,
    pub information_count: i64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Severity {
    Error,
    Warning,
    Information,
}