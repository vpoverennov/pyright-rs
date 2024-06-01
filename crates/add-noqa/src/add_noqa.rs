use std::collections::HashMap;
use std::fs;
use std::fs::{File};
use std::io::{BufRead, BufReader, BufWriter, Read};
use std::io::Write;
use std::path::Path;

use itertools::Itertools;

use pyright::Diagnostic;
use regex::Regex;

fn add_ignores_to_file<'a, T: IntoIterator<Item=&'a Diagnostic>>(file_path: &Path, diagnostics: T) -> anyhow::Result<()> {
    let rules: Vec<&String> = diagnostics.into_iter().filter_map(|d| d.rule.as_ref()).unique().collect();

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

    final_diagnostic_state.reserve(rules.len());
    for string_ref in rules {
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

fn add_ignores_to_lines<'a, T: IntoIterator<Item=&'a Diagnostic>>(file_path: &Path, diagnostics: T) -> anyhow::Result<()> {
    let mut rules_by_line: HashMap<u64, Vec<&String>> = HashMap::new();

    for item in diagnostics {
        if let Some(rule) = &item.rule {
            rules_by_line
                .entry(item.range.start.line)
                .or_insert_with(Vec::new)
                .push(rule);
        }
    }

    let file = fs::File::open(file_path)?;
    let reader = BufReader::new(file);

    let new_lines:Vec<String> = reader.lines().enumerate().map(|(line_number, line)| {
        let line = line.unwrap();
        if let Some(rules) = rules_by_line.get(&(line_number as u64)) {
            let ignore_comment =
                rules
                    .iter()
                    .unique()
                    .sorted()
                    .join(",");
            format!("{line}  # pyright: ignore [{ignore_comment}]")
        } else {
            line.to_string()
        }
    }).collect();

    fs::write(file_path, new_lines.join("\n"))?;

    Ok(())
}

pub fn apply_ignores<'a, I>(diagnostics: I, inline: bool) -> anyhow::Result<()>
    where
        I: IntoIterator<Item=&'a Diagnostic>,
{
    for (file_path, group) in &diagnostics.into_iter().group_by(|diagnostic| &diagnostic.file) {
        if inline {
            add_ignores_to_lines(&file_path, group)?;
        } else {
            add_ignores_to_file(&file_path, group)?;
        }
    }
    Ok(())
}
