# RAG Architecture - 检索增强生成

Layout: Left-to-right flow with two parallel tracks merging

INDEXING TRACK (top):
- 文档 (Documents) → 切分 (Chunking) → 嵌入 (Embedding) → 向量存储 (Vector Store)

QUERY TRACK (bottom):
- 用户问题 (User Query) → 嵌入 (Embedding) → 相似度搜索 (Similarity Search)

MERGE POINT:
- 向量存储 connects to 相似度搜索
- 检索结果 (Retrieved Context) → LLM → 生成回答 (Generated Answer)

LABELS:
- R = Retrieval (检索)
- A = Augmented (增强)
- G = Generation (生成)

VISUAL ELEMENTS:
- Simple boxes with rounded corners
- Thin arrows showing data flow
- Dashed line separating indexing from querying phase
- Small database icon for vector store

STYLE: Notion-style minimalist hand-drawn line art, clean black strokes, subtle shading, educational diagram feel

ASPECT: 16:9
