use anyhow::{Context, Result, bail};
use std::fs::{self, File};
use std::io::{Read, Seek, SeekFrom, Write};
use std::path::Path;

const EMBEDDED_SB3_MAGIC: &[u8] = b"SNR_EMBED_SB3_V1\0";
const EMBEDDED_SB3_SIZE_BYTES: usize = 8;
const EMBEDDED_SB3_TRAILER_BYTES: usize = EMBEDDED_SB3_SIZE_BYTES + EMBEDDED_SB3_MAGIC.len();

pub fn emit_embedded_project_executable(
    template_executable: &Path,
    sb3_path: &Path,
    output_path: &Path,
) -> Result<()> {
    if output_path == template_executable {
        bail!(
            "refusing to overwrite running template executable: {}",
            output_path.display()
        );
    }

    let mut template = File::open(template_executable).with_context(|| {
        format!(
            "failed to open template executable: {}",
            template_executable.display()
        )
    })?;
    let mut sb3_file = File::open(sb3_path)
        .with_context(|| format!("failed to open sb3 file: {}", sb3_path.display()))?;

    if let Some(parent) = output_path.parent() {
        if !parent.as_os_str().is_empty() {
            fs::create_dir_all(parent).with_context(|| {
                format!(
                    "failed to create output directory for executable: {}",
                    parent.display()
                )
            })?;
        }
    }

    let mut output = File::create(output_path).with_context(|| {
        format!(
            "failed to create output executable: {}",
            output_path.display()
        )
    })?;

    std::io::copy(&mut template, &mut output).with_context(|| {
        format!(
            "failed to copy template executable into output: {}",
            output_path.display()
        )
    })?;

    let sb3_size_u64 = std::io::copy(&mut sb3_file, &mut output)
        .with_context(|| format!("failed to append sb3 payload from {}", sb3_path.display()))?;
    if sb3_size_u64 == 0 {
        bail!("sb3 payload is empty: {}", sb3_path.display());
    }

    output
        .write_all(&sb3_size_u64.to_le_bytes())
        .context("failed to write embedded sb3 size trailer")?;
    output
        .write_all(EMBEDDED_SB3_MAGIC)
        .context("failed to write embedded sb3 trailer magic")?;

    let template_permissions = fs::metadata(template_executable)
        .with_context(|| {
            format!(
                "failed to read template executable metadata: {}",
                template_executable.display()
            )
        })?
        .permissions();
    fs::set_permissions(output_path, template_permissions).with_context(|| {
        format!(
            "failed to set executable permissions on {}",
            output_path.display()
        )
    })?;

    Ok(())
}

pub fn read_embedded_project_bytes(executable_path: &Path) -> Result<Option<Vec<u8>>> {
    let mut executable = File::open(executable_path)
        .with_context(|| format!("failed to open executable: {}", executable_path.display()))?;
    let executable_len = executable
        .metadata()
        .with_context(|| {
            format!(
                "failed to read executable metadata: {}",
                executable_path.display()
            )
        })?
        .len();

    if executable_len < EMBEDDED_SB3_TRAILER_BYTES as u64 {
        return Ok(None);
    }

    executable
        .seek(SeekFrom::End(-(EMBEDDED_SB3_TRAILER_BYTES as i64)))
        .context("failed to seek to embedded sb3 trailer")?;
    let mut trailer = [0u8; EMBEDDED_SB3_TRAILER_BYTES];
    executable
        .read_exact(&mut trailer)
        .context("failed to read embedded sb3 trailer")?;

    if trailer[EMBEDDED_SB3_SIZE_BYTES..] != EMBEDDED_SB3_MAGIC[..] {
        return Ok(None);
    }

    let mut size_bytes = [0u8; EMBEDDED_SB3_SIZE_BYTES];
    size_bytes.copy_from_slice(&trailer[..EMBEDDED_SB3_SIZE_BYTES]);
    let sb3_size = u64::from_le_bytes(size_bytes);
    let payload_start = executable_len
        .checked_sub(EMBEDDED_SB3_TRAILER_BYTES as u64)
        .and_then(|offset| offset.checked_sub(sb3_size))
        .ok_or_else(|| {
            anyhow::anyhow!(
                "embedded sb3 trailer is invalid or truncated: {}",
                executable_path.display()
            )
        })?;

    executable
        .seek(SeekFrom::Start(payload_start))
        .context("failed to seek to embedded sb3 payload")?;
    let payload_len = usize::try_from(sb3_size).map_err(|_| {
        anyhow::anyhow!(
            "embedded sb3 payload is too large for this platform: {} bytes",
            sb3_size
        )
    })?;
    let mut payload = vec![0u8; payload_len];
    executable
        .read_exact(&mut payload)
        .context("failed to read embedded sb3 payload")?;

    Ok(Some(payload))
}
