# PDF-QA System Architecture - Complete Pipeline

Layout: Horizontal flow, left to right with clear stages

STAGE 1 - INPUT:
- PDF文件 (PDF icon)

STAGE 2 - PROCESSING:
- 解析 (Parse) → 切分 (Chunk) → 嵌入 (Embed)

STAGE 3 - STORAGE:
- 向量索引 (Vector Index) - database cylinder icon

STAGE 4 - QUERY (bidirectional arrow to storage):
- 用户问题 (User Question) → 语义搜索 (Semantic Search)

STAGE 5 - GENERATION:
- 上下文 (Context) + 问题 → LLM (brain/AI icon)

STAGE 6 - OUTPUT:
- 回答 (Answer) - chat bubble icon

ANNOTATIONS:
- "Rig Framework" label spanning stages 3-5
- "OpenAI API" small label near LLM
- Technology badges: Rust, Tokio, pdf-extract

STYLE: Notion-style minimalist hand-drawn line art, clean documentation-ready diagram, professional yet friendly, suitable for README

ASPECT: 16:9
