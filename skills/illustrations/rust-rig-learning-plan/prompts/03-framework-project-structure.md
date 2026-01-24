# PDF-QA Project Structure

Layout: Hierarchical tree diagram, top-down

ROOT:
- pdf-qa/ (main project folder)

LEVEL 1 - Entry Points:
- main.rs (CLI入口)
- lib.rs (库入口)

LEVEL 2 - Modules (three columns):
Column 1: pdf/
  - mod.rs
  - parser.rs → "PDF解析"

Column 2: index/
  - mod.rs
  - chunker.rs → "文本切分"
  - store.rs → "向量存储"

Column 3: agent/
  - mod.rs
  - qa.rs → "问答Agent"

SHARED:
- error.rs (错误类型) - connects to all modules

VISUAL ELEMENTS:
- Folder icons for directories
- File icons for .rs files
- Dotted lines showing module dependencies
- Small annotation bubbles for Chinese descriptions

STYLE: Notion-style minimalist hand-drawn, clean hierarchy visualization, monospace-like font for filenames, light connecting lines

ASPECT: 16:9
