#!/usr/bin/env python3
from __future__ import annotations

import html
import posixpath
import re
from pathlib import Path

ROOT = Path(__file__).resolve().parent
SITE_CONTENT = ROOT / "site" / "content"
REPO_README_URL = "https://github.com/pnsk-lab/xyo-rust/blob/main/README.md"

PAGES = [
    ("README.md", "index.xml", "index.html", "はじめに"),
    ("getting-started.md", "getting-started/index.xml", "getting-started/index.html", "セットアップ"),
    ("cli.md", "cli/index.xml", "cli/index.html", "CLI"),
    ("blocks.md", "blocks/index.xml", "blocks/index.html", "対応ブロック一覧"),
    ("architecture.md", "architecture/index.xml", "architecture/index.html", "アーキテクチャ"),
]

PAGE_WEB_PATHS = {source: web_path for source, _site_path, web_path, _label in PAGES}

LEGACY_OUTPUTS = [
    "getting-started.xml",
    "cli.xml",
    "blocks.xml",
    "architecture.xml",
    "getting-started.html",
    "cli.html",
    "blocks.html",
    "architecture.html",
]

LEGACY_DIRS = ["getting-started", "cli", "blocks", "architecture"]

LIST_RE = re.compile(r"^(\s*)([-+*]|\d+\.)\s+(.*)$")
INLINE_RE = re.compile(r"`([^`]+)`|\[([^\]]+)\]\(([^)]+)\)|\*\*([^*]+)\*\*|\*([^*]+)\*")
TABLE_SEPARATOR_RE = re.compile(r"^\s*\|?(?:\s*:?-+:?\s*\|)+\s*:?-+:?\s*\|?\s*$")


def finalize_relative_link(path: str) -> str:
    if path.endswith("/index.html"):
        return path[: -len("index.html")]
    return path


def rewrite_link(url: str, current_source: str) -> str:
    url = url.strip()
    if url.startswith(("http://", "https://", "#", "mailto:")):
        return url
    if url == "../README.md":
        return REPO_README_URL

    suffix = ""
    if "#" in url:
        url, fragment = url.split("#", 1)
        suffix = f"#{fragment}"

    path = Path(url)
    if path.suffix == ".md":
        normalized = path.name
        if normalized not in PAGE_WEB_PATHS:
            target = path.with_suffix(".html").as_posix()
        else:
            target = PAGE_WEB_PATHS[normalized]
    else:
        target = path.as_posix()

    current_dir = posixpath.dirname(PAGE_WEB_PATHS[current_source]) or "."
    relative = posixpath.relpath(target, start=current_dir)
    return finalize_relative_link(relative) + suffix


def strip_inline_markdown(text: str) -> str:
    text = re.sub(r"`([^`]+)`", r"\1", text)
    text = re.sub(r"\[([^\]]+)\]\(([^)]+)\)", r"\1", text)
    text = re.sub(r"\*\*([^*]+)\*\*", r"\1", text)
    text = re.sub(r"\*([^*]+)\*", r"\1", text)
    return text.strip()


def render_inline(text: str, current_source: str) -> str:
    parts: list[str] = []
    last = 0
    for match in INLINE_RE.finditer(text):
        parts.append(html.escape(text[last : match.start()]))
        code, link_text, link_url, bold_text, em_text = match.groups()
        if code is not None:
            parts.append(f"<code>{html.escape(code)}</code>")
        elif link_text is not None:
            href = html.escape(rewrite_link(link_url, current_source), quote=True)
            parts.append(f"<a href=\"{href}\">{render_inline(link_text, current_source)}</a>")
        elif bold_text is not None:
            parts.append(f"<strong>{render_inline(bold_text, current_source)}</strong>")
        elif em_text is not None:
            parts.append(f"<em>{render_inline(em_text, current_source)}</em>")
        last = match.end()
    parts.append(html.escape(text[last:]))
    return "".join(parts)


def split_table_row(line: str) -> list[str]:
    raw = line.strip()
    if raw.startswith("|"):
        raw = raw[1:]
    if raw.endswith("|"):
        raw = raw[:-1]
    return [cell.strip() for cell in raw.split("|")]


def render_code_block(lines: list[str]) -> str:
    escaped = "<br />".join(html.escape(line) for line in lines)
    return f"<blockquote><code>{escaped}</code></blockquote>"


def render_table(lines: list[str], current_source: str) -> str:
    header = split_table_row(lines[0])
    rows = [split_table_row(line) for line in lines[2:]]
    out = ["<table>", "<tr>"]
    out.extend(f"<th>{render_inline(cell, current_source)}</th>" for cell in header)
    out.append("</tr>")
    for row in rows:
        out.append("<tr>")
        out.extend(f"<td>{render_inline(cell, current_source)}</td>" for cell in row)
        out.append("</tr>")
    out.append("</table>")
    return "\n".join(out)


def render_paragraph(lines: list[str], current_source: str) -> str:
    pieces: list[str] = []
    pending_break = False
    for line in lines:
        content = line.rstrip()
        if not content:
            continue
        rendered = render_inline(content.rstrip(), current_source)
        if not pieces:
            pieces.append(rendered)
        elif pending_break:
            pieces.append("<br />" + rendered)
        else:
            pieces.append(" " + rendered)
        pending_break = line.endswith("  ")
    return f"<p>{''.join(pieces)}</p>"


def render_quote(lines: list[str], current_source: str) -> str:
    quote_lines = [re.sub(r"^>\s?", "", line) for line in lines]
    paragraphs: list[list[str]] = []
    current: list[str] = []
    for line in quote_lines:
        if line.strip() == "":
            if current:
                paragraphs.append(current)
                current = []
            continue
        current.append(line)
    if current:
        paragraphs.append(current)

    tag = "note"
    if paragraphs:
        first = strip_inline_markdown(" ".join(paragraphs[0]))
        for prefix, mapped in (("注意", "warning"), ("補足", "note")):
            if first.startswith(prefix):
                tag = mapped
                cleaned = re.sub(rf"^\*\*?{prefix}\*\*?[:：]?\s*", "", " ".join(paragraphs[0])).strip()
                paragraphs[0] = [cleaned] if cleaned else []
                break
    rendered = [render_paragraph(p, current_source) for p in paragraphs if any(part.strip() for part in p)]
    return f"<{tag}>\n" + "\n".join(rendered) + f"\n</{tag}>"


def parse_list(lines: list[str], index: int, current_source: str) -> tuple[str, int]:
    match = LIST_RE.match(lines[index])
    assert match is not None
    base_indent = len(match.group(1))
    ordered = match.group(2).endswith(".")
    tag = "ol" if ordered else "ul"
    items: list[str] = []
    i = index

    while i < len(lines):
        line = lines[i]
        current = LIST_RE.match(line)
        if current is None or len(current.group(1)) != base_indent or current.group(2).endswith(".") != ordered:
            break

        text = current.group(3).rstrip()
        i += 1
        nested_parts: list[str] = []
        while i < len(lines) and lines[i].strip() == "":
            i += 1
            break

        body = render_inline(text, current_source)
        if i < len(lines):
            child = LIST_RE.match(lines[i])
            if child is not None and len(child.group(1)) > base_indent:
                nested_xml, i = parse_list(lines, i, current_source)
                nested_parts.append(nested_xml)
        if nested_parts:
            body += "\n" + "\n".join(nested_parts)
        items.append(f"<li>{body}</li>")

    return "<{}>\n{}\n</{}>".format(tag, "\n".join(items), tag), i


def is_table_start(lines: list[str], index: int) -> bool:
    if index + 1 >= len(lines):
        return False
    return "|" in lines[index] and TABLE_SEPARATOR_RE.match(lines[index + 1]) is not None


def is_block_start(line: str) -> bool:
    stripped = line.lstrip()
    return (
        stripped.startswith("### ")
        or stripped.startswith("#### ")
        or stripped.startswith(">")
        or stripped.startswith("```")
        or LIST_RE.match(line) is not None
        or stripped.startswith("---")
    )


def render_blocks(lines: list[str], current_source: str) -> str:
    out: list[str] = []
    i = 0
    while i < len(lines):
        line = lines[i]
        stripped = line.strip()
        if not stripped:
            i += 1
            continue

        if line.startswith("### "):
            out.append(f"<h3>{render_inline(line[4:].strip(), current_source)}</h3>")
            i += 1
            continue

        if line.startswith("#### "):
            out.append(f"<h4>{render_inline(line[5:].strip(), current_source)}</h4>")
            i += 1
            continue

        if stripped.startswith(">"):
            start = i
            while i < len(lines) and lines[i].lstrip().startswith(">"):
                i += 1
            out.append(render_quote(lines[start:i], current_source))
            continue

        if stripped.startswith("```"):
            i += 1
            code_lines: list[str] = []
            while i < len(lines) and not lines[i].strip().startswith("```"):
                code_lines.append(lines[i])
                i += 1
            if i < len(lines):
                i += 1
            out.append(render_code_block(code_lines))
            continue

        if is_table_start(lines, i):
            start = i
            i += 2
            while i < len(lines) and "|" in lines[i] and lines[i].strip():
                i += 1
            out.append(render_table(lines[start:i], current_source))
            continue

        if LIST_RE.match(line):
            xml, i = parse_list(lines, i, current_source)
            out.append(xml)
            continue

        if stripped == "---":
            out.append("<hr />")
            i += 1
            continue

        para_lines = [line]
        i += 1
        while i < len(lines) and lines[i].strip() and not is_block_start(lines[i]) and not is_table_start(lines, i):
            para_lines.append(lines[i])
            i += 1
        out.append(render_paragraph(para_lines, current_source))

    return "\n".join(out)


def split_sections(lines: list[str]) -> tuple[list[str], list[tuple[str, list[str]]]]:
    intro: list[str] = []
    sections: list[tuple[str, list[str]]] = []
    current_title: str | None = None
    current_lines: list[str] = []

    for line in lines:
        if line.startswith("## "):
            if current_title is None:
                intro = current_lines
            else:
                sections.append((current_title, current_lines))
            current_title = line[3:].strip()
            current_lines = []
            continue
        current_lines.append(line)

    if current_title is None:
        intro = current_lines
    else:
        sections.append((current_title, current_lines))

    return intro, sections


def build_document(page_title: str, body_intro: list[str], sections: list[tuple[str, list[str]]], current_source: str) -> str:
    out = ["<?xml version=\"1.0\"?>", "<document>", "\t<header>", f"\t\t<title>{html.escape(page_title)}</title>", "\t</header>", "\t<body>"]

    intro = render_blocks(body_intro, current_source).strip()
    if intro:
        for line in intro.splitlines():
            out.append(f"\t\t{line}")

    for index, (section_title, section_lines) in enumerate(sections, start=1):
        section_id = f"section-{index}"
        out.append(f"\t\t<section id=\"{section_id}\" title=\"{html.escape(strip_inline_markdown(section_title), quote=True)}\">")
        rendered = render_blocks(section_lines, current_source).strip()
        if rendered:
            for line in rendered.splitlines():
                out.append(f"\t\t\t{line}")
        out.append("\t\t</section>")

    out.extend(["\t</body>", "</document>", ""])
    return "\n".join(out)


def convert_page(source_name: str, output_name: str, _web_path: str, page_label: str) -> None:
    source = ROOT / source_name
    lines = source.read_text(encoding="utf-8").splitlines()
    if not lines or not lines[0].startswith("# "):
        raise ValueError(f"{source_name} must start with a level-1 heading")

    page_title = strip_inline_markdown(lines[0][2:].strip()) or page_label
    intro, sections = split_sections(lines[1:])
    xml = build_document(page_title, intro, sections, source_name)
    destination = SITE_CONTENT / output_name
    destination.parent.mkdir(parents=True, exist_ok=True)
    destination.write_text(xml, encoding="utf-8")


def main() -> None:
    SITE_CONTENT.mkdir(parents=True, exist_ok=True)
    for legacy_dir in LEGACY_DIRS:
        legacy_path = SITE_CONTENT / legacy_dir
        if legacy_path.exists() and legacy_path.is_dir():
            for child in sorted(legacy_path.rglob("*"), reverse=True):
                if child.is_file():
                    child.unlink()
                elif child.is_dir():
                    child.rmdir()
            legacy_path.rmdir()
    for legacy in LEGACY_OUTPUTS:
        legacy_path = SITE_CONTENT / legacy
        if legacy_path.exists():
            legacy_path.unlink()
    for source_name, output_name, web_path, page_label in PAGES:
        convert_page(source_name, output_name, web_path, page_label)
    print("Generated Taiga XML from Markdown.")


if __name__ == "__main__":
    main()
