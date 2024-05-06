use std::collections::HashMap;
use std::fs::{File};
use std::io::{BufRead, BufReader, BufWriter, Read};
use std::io::Write;
use std::path::Path;

use itertools::Itertools;

use pyright::Diagnostic;
use regex::Regex;

fn add_ignores_to_file<P: AsRef<Path>>(file_path: P, ignores: &Vec<&String>) -> anyhow::Result<()> {
    let file = File::open(&file_path)?;
    let mut reader = BufReader::new(file);
    let mut first_line = String::new();
    reader.read_line(&mut first_line)?;
    let mut buf = Vec::new();
    reader.read_to_end(&mut buf)?;

    let mut need_replace = false;
    let mut final_diagnostic_state: HashMap<String, bool> = HashMap::new();
    if let Some(pyright_config) = first_line.strip_prefix("# pyright: ") {
        let re = Regex::new(r"(\w+)=(true|false)").unwrap();
        for capture in re.captures_iter(pyright_config) {
            let key = capture.get(1).unwrap().as_str();
            let value = capture.get(2).unwrap().as_str();
            final_diagnostic_state.insert(key.to_string(), value.parse::<bool>().unwrap());
        }
        need_replace = true
    }

    final_diagnostic_state.reserve(ignores.len());
    for string_ref in ignores {
        final_diagnostic_state.insert((*string_ref).clone(), false);
    }
    let diagnostic_comment =
        final_diagnostic_state
            .iter()
            .sorted_by(|a, b| a.0.cmp(b.0))
            .map(|(d, v)| format!("{d}={v}"))
            .join(",");

    let output_file = File::create(&file_path)?;
    let mut writer = BufWriter::new(output_file);

    let pyright_line = format!("# pyright: {diagnostic_comment}");
    writeln!(writer, "{pyright_line}")?;
    if !need_replace {
        write!(writer, "{first_line}")?;
    }
    writer.write_all(&buf)?;

    Ok(())
}

pub fn apply_ignores<'a, I>(diagnostics: I) -> anyhow::Result<()>
    where
        I: IntoIterator<Item=&'a Diagnostic>,
{
    for (file_path, group) in &diagnostics.into_iter().group_by(|diagnostic| &diagnostic.file) {
        let rules: Vec<&String> = group.filter_map(|d| d.rule.as_ref()).unique().collect();
        add_ignores_to_file(file_path, &rules)?;
    }
    Ok(())
}
