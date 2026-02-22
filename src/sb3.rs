use zip::ZipArchive;

use crate::types::{self, ScratchProject};

use std::{error::Error, fmt, fs::File, io::Read, string::FromUtf8Error};

#[derive(Debug)]
pub enum ReadSb3Error {
    OpenFile {
        path: String,
        source: std::io::Error,
    },
    OpenZip {
        path: String,
        source: zip::result::ZipError,
    },
    MissingProjectJson {
        path: String,
        source: zip::result::ZipError,
    },
    ReadProjectJson {
        path: String,
        source: std::io::Error,
    },
    ParseProjectJson {
        path: String,
        source: serde_json::Error,
        json_path: Option<String>,
        context: String,
    },
    ReadAsUTF8 {
        path: String,
        source: FromUtf8Error,
    },
}

impl fmt::Display for ReadSb3Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::OpenFile { path, .. } => {
                write!(f, "Failed to open SB3 file: `{path}`")
            }
            Self::OpenZip { path, .. } => {
                write!(f, "Failed to read as SB3 (ZIP): `{path}`")
            }
            Self::MissingProjectJson { path, .. } => {
                write!(f, "`project.json` not found in archive: `{path}`")
            }
            Self::ReadProjectJson { path, .. } => {
                write!(f, "Failed to read `project.json`: `{path}`")
            }
            Self::ParseProjectJson {
                path,
                json_path,
                context,
                ..
            } => {
                if let Some(json_path) = json_path {
                    write!(
                        f,
                        "Failed to parse `project.json`: `{path}`\nPath: {json_path}\n{context}"
                    )
                } else {
                    write!(f, "Failed to parse `project.json`: `{path}`\n{context}")
                }
            }
            Self::ReadAsUTF8 { path, .. } => {
                write!(f, "Can't open project.json with utf-8: {path}")
            }
        }
    }
}

impl Error for ReadSb3Error {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::OpenFile { source, .. } => Some(source),
            Self::OpenZip { source, .. } => Some(source),
            Self::MissingProjectJson { source, .. } => Some(source),
            Self::ReadProjectJson { source, .. } => Some(source),
            Self::ParseProjectJson { source, .. } => Some(source),
            Self::ReadAsUTF8 { source, .. } => Some(source),
        }
    }
}

fn clipped_line(line_text: &str, column: usize, radius: usize) -> (String, usize) {
    let chars: Vec<char> = line_text.chars().collect();
    let caret_pos = column.saturating_sub(1).min(chars.len());
    let start = caret_pos.saturating_sub(radius);
    let end = (caret_pos + radius).min(chars.len());

    let left_clipped = start > 0;
    let right_clipped = end < chars.len();
    let mut snippet = String::new();

    if left_clipped {
        snippet.push_str("...");
    }
    snippet.push_str(&chars[start..end].iter().collect::<String>());
    if right_clipped {
        snippet.push_str("...");
    }

    let pointer_pos = (if left_clipped { 3 } else { 0 }) + caret_pos.saturating_sub(start);
    (snippet, pointer_pos)
}

fn json_error_context(input: &str, line: usize, column: usize, radius: usize) -> String {
    let lines: Vec<&str> = input.lines().collect();
    if lines.is_empty() {
        return format!("Location: line {line}, column {column}");
    }

    let line_index = line.saturating_sub(1).min(lines.len().saturating_sub(1));
    let context_start = line_index.saturating_sub(1);
    let context_end = (line_index + 1).min(lines.len().saturating_sub(1));
    let width = (context_end + 1).to_string().len().max(2);

    let mut out = String::new();
    out.push_str(&format!("Location: line {line}, column {column}\n"));
    out.push_str("Context:\n");

    for i in context_start..=context_end {
        let current_line = lines[i];
        if i == line_index {
            let (snippet, pointer_pos) = clipped_line(current_line, column, radius);
            out.push_str(&format!("{:>width$} | {}\n", i + 1, snippet, width = width));
            out.push_str(&format!(
                "{:>width$} | {}^\n",
                "",
                " ".repeat(pointer_pos),
                width = width
            ));
        } else {
            let chars: Vec<char> = current_line.chars().collect();
            let display = if chars.len() > radius * 2 {
                format!("{}...", chars[..radius * 2].iter().collect::<String>())
            } else {
                current_line.to_string()
            };
            out.push_str(&format!("{:>width$} | {}\n", i + 1, display, width = width));
        }
    }

    out.trim_end().to_string()
}

fn refine_json_path(input: &str, json_path: &str) -> Option<String> {
    if !(json_path.starts_with("targets[") && json_path.ends_with(']')) {
        return None;
    }

    let index_text = &json_path["targets[".len()..json_path.len() - 1];
    let target_index = index_text.parse::<usize>().ok()?;

    let root: serde_json::Value = serde_json::from_str(input).ok()?;
    let targets = root.get("targets")?.as_array()?;
    let target = targets.get(target_index)?;
    let blocks = target.get("blocks")?.as_object()?;

    for (block_index, (block_id, block_value)) in blocks.iter().enumerate() {
        if serde_json::from_value::<types::BlockAndTopLevelPrimitive>(block_value.clone()).is_err()
        {
            return Some(format!(
                "targets[{target_index}].blocks[{block_index}] (id: {block_id})"
            ));
        }
    }

    None
}

pub fn read_json(path: &str) -> Result<String, ReadSb3Error> {
    let stream = File::open(path).map_err(|source| ReadSb3Error::OpenFile {
        path: path.to_string(),
        source,
    })?;
    let mut archive = ZipArchive::new(stream).map_err(|source| ReadSb3Error::OpenZip {
        path: path.to_string(),
        source,
    })?;
    let mut entry =
        archive
            .by_name("project.json")
            .map_err(|source| ReadSb3Error::MissingProjectJson {
                path: path.to_string(),
                source,
            })?;
    let mut buf = Vec::with_capacity(entry.size() as usize);
    entry
        .read_to_end(&mut buf)
        .map_err(|source| ReadSb3Error::ReadProjectJson {
            path: path.to_string(),
            source,
        })?;
    let str = String::from_utf8(buf).map_err(|source| ReadSb3Error::ReadAsUTF8 {
        path: path.to_string(),
        source: source,
    })?;
    Ok(str)
}
pub fn read_sb3(path: &str) -> Result<ScratchProject, ReadSb3Error> {
    let stream = File::open(path).map_err(|source| ReadSb3Error::OpenFile {
        path: path.to_string(),
        source,
    })?;
    let mut archive = ZipArchive::new(stream).map_err(|source| ReadSb3Error::OpenZip {
        path: path.to_string(),
        source,
    })?;
    let mut entry =
        archive
            .by_name("project.json")
            .map_err(|source| ReadSb3Error::MissingProjectJson {
                path: path.to_string(),
                source,
            })?;
    let mut buf = Vec::with_capacity(entry.size() as usize);
    entry
        .read_to_end(&mut buf)
        .map_err(|source| ReadSb3Error::ReadProjectJson {
            path: path.to_string(),
            source,
        })?;
    let mut deserializer = serde_json::Deserializer::from_slice(&buf);
    let project: ScratchProject = match serde_path_to_error::deserialize(&mut deserializer) {
        Ok(project) => project,
        Err(source) => {
            let json_path = source.path().to_string();
            let source = source.into_inner();
            let line = source.line();
            let column = source.column();
            let json_str = String::from_utf8_lossy(&buf);
            let refined_json_path = if json_path.is_empty() {
                None
            } else {
                refine_json_path(json_str.as_ref(), &json_path).or(Some(json_path))
            };
            return Err(ReadSb3Error::ParseProjectJson {
                path: path.to_string(),
                source,
                json_path: refined_json_path,
                context: json_error_context(json_str.as_ref(), line, column, 60),
            });
        }
    };
    Ok(project)
}
