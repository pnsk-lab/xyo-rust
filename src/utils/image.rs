//! Image and SVG processing utilities.

use crate::project::sb3;
use anyhow::{Context, Result, bail};
use std::io::Write;
use std::process::{Command, Stdio};

/// Decode a costume into RGBA pixel data.
///
/// # Arguments
/// * `costume` - The costume metadata
/// * `bytes` - The raw image/SVG data
///
/// # Returns
/// A tuple of `(width, height, rgba_pixels)`.
pub fn decode_costume_rgba(
    costume: &sb3::Costume,
    bytes: &[u8],
) -> Result<(usize, usize, Vec<u8>)> {
    let format = costume.data_format.trim().to_ascii_lowercase();
    if format == "svg" {
        let png_bytes = rasterize_svg_with_convert(bytes)
            .context("SVG decoding requires ImageMagick `convert` with SVG support")?;
        let image = image::load_from_memory(&png_bytes)
            .context("failed to decode rasterized SVG image data")?
            .to_rgba8();
        let (width, height) = image.dimensions();
        return Ok((width as usize, height as usize, image.into_raw()));
    }

    let image = image::load_from_memory(bytes)
        .with_context(|| format!("unsupported image format '{}'", costume.data_format))?
        .to_rgba8();
    let (width, height) = image.dimensions();
    Ok((width as usize, height as usize, image.into_raw()))
}

/// Rasterize an SVG file using ImageMagick's `convert` command.
///
/// # Arguments
/// * `svg_bytes` - The raw SVG file content
///
/// # Returns
/// PNG-encoded bytes.
fn rasterize_svg_with_convert(svg_bytes: &[u8]) -> Result<Vec<u8>> {
    let sanitized_svg = sanitize_svg_for_convert(&String::from_utf8_lossy(svg_bytes));
    let mut child = Command::new("convert")
        // Scratch SVG costumes rely on transparent background; preserve alpha.
        .arg("-background")
        .arg("none")
        .arg("svg:-")
        .arg("png:-")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .context("failed to launch `convert` process")?;

    if let Some(stdin) = child.stdin.as_mut() {
        stdin
            .write_all(sanitized_svg.as_bytes())
            .context("failed to feed SVG bytes to `convert`")?;
    } else {
        bail!("`convert` stdin was not available");
    }

    let output = child
        .wait_with_output()
        .context("failed to wait for `convert`")?;
    if output.stdout.is_empty() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        if !output.status.success() {
            bail!("`convert` failed: {}", stderr.trim());
        }
        bail!("`convert` produced an empty image");
    }
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        eprintln!(
            "warning: `convert` returned non-zero status while rasterizing SVG: {}",
            stderr.trim()
        );
    }
    Ok(output.stdout)
}

/// Sanitize SVG content for ImageMagick conversion.
///
/// Removes attributes that may cause conversion issues.
fn sanitize_svg_for_convert(svg: &str) -> String {
    svg.replace("stroke-dasharray=\"\"", "")
        .replace("stroke-dashoffset=\"0\"", "")
        .replace("style=\"mix-blend-mode: normal\"", "")
}
