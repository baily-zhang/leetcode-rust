# Rust + Rig 学习计划

> 目标：4-5 周内完成学习，产出一个可放入简历的 RAG 项目
>
> 每日投入：3-4 小时
>
> 前置条件：已掌握 Rust 基础语法、traits、泛型

---

## 第一周：异步 Rust 基础

### Day 1: 理解 async/await 概念

**学习内容：**
- [x] 什么是 Future
- [x] async fn 返回什么
- [x] .await 做了什么
- [x] 为什么需要运行时（Runtime）

**资源：**
- 阅读：https://rust-lang.github.io/async-book/01_getting_started/01_chapter.html
- 阅读：https://rust-lang.github.io/async-book/01_getting_started/04_async_await_primer.html

**练习：**
```rust
// 写一个 async 函数，理解它返回的类型
async fn hello() -> String {
    "Hello".to_string()
}

fn main() {
    let future = hello(); // 这是什么类型？
    // 尝试打印 future 的类型
}
```

**完成标准：**
- [x] 能解释 Future trait 的基本结构
- [x] 理解 lazy evaluation（惰性求值）

---

### Day 2: Tokio 入门

**学习内容：**
- [ ] 安装和配置 Tokio
- [ ] `#[tokio::main]` 宏的作用
- [ ] tokio::spawn 创建任务
- [ ] tokio::time::sleep vs std::thread::sleep

**资源：**
- 官方教程：https://tokio.rs/tokio/tutorial

**练习项目：创建一个简单的异步计时器**
```bash
cargo new async-timer
cd async-timer
```

```toml
# Cargo.toml
[dependencies]
tokio = { version = "1", features = ["full"] }
```

```rust
// src/main.rs
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    println!("开始计时...");

    // 任务1：3秒后打印
    let task1 = tokio::spawn(async {
        sleep(Duration::from_secs(3)).await;
        println!("任务1完成（3秒）");
    });

    // 任务2：1秒后打印
    let task2 = tokio::spawn(async {
        sleep(Duration::from_secs(1)).await;
        println!("任务2完成（1秒）");
    });

    // 等待两个任务
    let _ = tokio::join!(task1, task2);
    println!("全部完成");
}
```

**完成标准：**
- [ ] 理解为什么任务2先完成（并发 vs 并行）
- [ ] 能解释 tokio::spawn 的作用

---

### Day 3: 异步 I/O

**学习内容：**
- [ ] tokio::fs 文件操作
- [ ] tokio::io 读写
- [ ] BufReader / BufWriter

**练习项目：异步文件读取器**
```rust
use tokio::fs::File;
use tokio::io::{AsyncBufReadExt, BufReader};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open("test.txt").await?;
    let reader = BufReader::new(file);
    let mut lines = reader.lines();

    while let Some(line) = lines.next_line().await? {
        println!("{}", line);
    }

    Ok(())
}
```

**扩展练习：**
- [ ] 同时读取多个文件，统计总行数
- [ ] 写一个异步的文件复制函数

**完成标准：**
- [ ] 理解 `?` 在 async 函数中的工作方式
- [ ] 能处理文件不存在的错误

---

### Day 4: HTTP 客户端 (reqwest)

**学习内容：**
- [ ] reqwest 库的使用
- [ ] GET/POST 请求
- [ ] JSON 序列化/反序列化（serde）

**练习项目：GitHub API 客户端**
```bash
cargo new github-client
```

```toml
# Cargo.toml
[dependencies]
tokio = { version = "1", features = ["full"] }
reqwest = { version = "0.12", features = ["json"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
```

```rust
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct GitHubUser {
    login: String,
    id: u64,
    public_repos: u32,
    followers: u32,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();

    let user: GitHubUser = client
        .get("https://api.github.com/users/rust-lang")
        .header("User-Agent", "rust-learning")
        .send()
        .await?
        .json()
        .await?;

    println!("用户: {}", user.login);
    println!("公开仓库: {}", user.public_repos);
    println!("粉丝: {}", user.followers);

    Ok(())
}
```

**扩展练习：**
- [ ] 添加命令行参数，查询任意用户
- [ ] 并发查询多个用户，比较响应时间

**完成标准：**
- [ ] 理解 serde 的 Deserialize 派生宏
- [ ] 能处理 API 返回的错误（404 等）

---

### Day 5: 错误处理深入

**学习内容：**
- [ ] 自定义错误类型
- [ ] thiserror 库
- [ ] anyhow 库
- [ ] 何时用 thiserror vs anyhow

**资源：**
- thiserror: https://docs.rs/thiserror/latest/thiserror/
- anyhow: https://docs.rs/anyhow/latest/anyhow/

**练习：重构 Day 4 的项目**
```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("HTTP 请求失败: {0}")]
    HttpError(#[from] reqwest::Error),

    #[error("用户 '{0}' 不存在")]
    UserNotFound(String),

    #[error("API 限流，请稍后重试")]
    RateLimited,
}

async fn get_user(username: &str) -> Result<GitHubUser, AppError> {
    let response = reqwest::get(format!(
        "https://api.github.com/users/{}",
        username
    )).await?;

    match response.status().as_u16() {
        200 => Ok(response.json().await?),
        404 => Err(AppError::UserNotFound(username.to_string())),
        403 => Err(AppError::RateLimited),
        _ => Err(AppError::HttpError(response.error_for_status().unwrap_err())),
    }
}
```

**完成标准：**
- [ ] 理解 #[from] 属性的作用
- [ ] 能解释 thiserror vs anyhow 的使用场景

---

### Day 6-7: 第一周项目 - 天气 CLI

**目标：** 整合本周所学，做一个命令行天气查询工具

**功能要求：**
- [ ] 接收城市名作为参数
- [ ] 调用天气 API（推荐 wttr.in 或 OpenWeatherMap）
- [ ] 显示温度、湿度、天气描述
- [ ] 优雅的错误处理
- [ ] 支持查询多个城市（并发）

**项目结构：**
```
weather-cli/
├── Cargo.toml
└── src/
    ├── main.rs      # 入口，参数解析
    ├── api.rs       # API 调用
    ├── error.rs     # 错误类型
    └── models.rs    # 数据结构
```

**依赖：**
```toml
[dependencies]
tokio = { version = "1", features = ["full"] }
reqwest = { version = "0.12", features = ["json"] }
serde = { version = "1", features = ["derive"] }
clap = { version = "4", features = ["derive"] }
thiserror = "2"
```

**完成标准：**
- [ ] 程序能正确运行
- [ ] 错误信息友好
- [ ] 代码结构清晰

---

## 第二周：Rig 基础

### Day 8: Rig 安装和第一个程序

**学习内容：**
- [ ] Rig 架构概览
- [ ] 设置 OpenAI API Key
- [ ] 第一个 Rig 程序

**准备工作：**
1. 获取 OpenAI API Key: https://platform.openai.com/api-keys
2. 设置环境变量：
   ```bash
   export OPENAI_API_KEY="sk-..."
   ```

**项目设置：**
```bash
cargo new rig-hello
cd rig-hello
```

```toml
# Cargo.toml
[dependencies]
rig-core = "0.9"
tokio = { version = "1", features = ["full"] }
```

```rust
// src/main.rs
use rig::providers::openai;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 从环境变量读取 API Key
    let client = openai::Client::from_env();

    // 创建一个 agent
    let agent = client
        .agent(openai::GPT_4O)
        .preamble("你是一个友好的助手，用中文回答问题。")
        .build();

    // 发送请求
    let response = agent.prompt("什么是 Rust 语言？").await?;

    println!("{}", response);

    Ok(())
}
```

**练习：**
- [ ] 修改 preamble，让它扮演不同角色
- [ ] 尝试不同的模型（GPT_4O_MINI 等）

**完成标准：**
- [ ] 程序成功运行并返回响应
- [ ] 理解 Client -> Agent -> Prompt 的流程

---

### Day 9: 理解 Rig 的 Agent 系统

**学习内容：**
- [ ] Agent vs Completion
- [ ] Agent Builder 模式
- [ ] Temperature、Max Tokens 等参数
- [ ] 多轮对话

**练习：带历史记录的对话**
```rust
use rig::providers::openai;
use rig::agent::Agent;
use std::io::{self, Write};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = openai::Client::from_env();
    let agent = client
        .agent(openai::GPT_4O_MINI)
        .preamble("你是一个 Rust 编程助手。")
        .temperature(0.7)
        .build();

    let mut history = Vec::new();

    loop {
        print!("你: ");
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let input = input.trim();

        if input == "quit" || input == "exit" {
            break;
        }

        // 构建带历史的 prompt
        let full_prompt = if history.is_empty() {
            input.to_string()
        } else {
            format!("{}\n\n用户: {}", history.join("\n"), input)
        };

        let response = agent.prompt(&full_prompt).await?;
        println!("助手: {}\n", response);

        history.push(format!("用户: {}\n助手: {}", input, response));
    }

    Ok(())
}
```

**完成标准：**
- [ ] 理解 temperature 参数的影响
- [ ] 能实现基本的多轮对话

---

### Day 10: Tool Use（工具调用）

**学习内容：**
- [ ] 什么是 Tool Use / Function Calling
- [ ] 在 Rig 中定义工具
- [ ] Agent 如何选择和调用工具

**练习：计算器工具**
```rust
use rig::providers::openai;
use rig::tool::Tool;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct CalculatorInput {
    operation: String,
    a: f64,
    b: f64,
}

#[derive(Serialize)]
struct CalculatorOutput {
    result: f64,
}

#[derive(Debug, thiserror::Error)]
#[error("计算错误: {0}")]
struct CalculatorError(String);

struct Calculator;

impl Tool for Calculator {
    const NAME: &'static str = "calculator";

    type Error = CalculatorError;
    type Args = CalculatorInput;
    type Output = CalculatorOutput;

    async fn definition(&self, _prompt: String) -> rig::tool::ToolDefinition {
        rig::tool::ToolDefinition {
            name: "calculator".to_string(),
            description: "执行基本数学运算（add, subtract, multiply, divide）".to_string(),
            parameters: serde_json::json!({
                "type": "object",
                "properties": {
                    "operation": {
                        "type": "string",
                        "enum": ["add", "subtract", "multiply", "divide"]
                    },
                    "a": { "type": "number" },
                    "b": { "type": "number" }
                },
                "required": ["operation", "a", "b"]
            }),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        let result = match args.operation.as_str() {
            "add" => args.a + args.b,
            "subtract" => args.a - args.b,
            "multiply" => args.a * args.b,
            "divide" => {
                if args.b == 0.0 {
                    return Err(CalculatorError("除数不能为零".to_string()));
                }
                args.a / args.b
            }
            _ => return Err(CalculatorError("未知操作".to_string())),
        };

        Ok(CalculatorOutput { result })
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = openai::Client::from_env();

    let agent = client
        .agent(openai::GPT_4O)
        .preamble("你是一个数学助手，可以使用计算器工具进行计算。")
        .tool(Calculator)
        .build();

    let response = agent.prompt("计算 123 乘以 456 是多少？").await?;
    println!("{}", response);

    Ok(())
}
```

**完成标准：**
- [ ] 理解 Tool trait 的结构
- [ ] 能添加新的工具函数

---

### Day 11: 向量嵌入（Embeddings）

**学习内容：**
- [ ] 什么是 Embedding
- [ ] 为什么需要 Embedding（语义搜索）
- [ ] Rig 中的 Embedding API

**练习：文本相似度计算**
```rust
use rig::providers::openai;
use rig::embeddings::EmbeddingsBuilder;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = openai::Client::from_env();

    let model = client.embedding_model(openai::TEXT_EMBEDDING_3_SMALL);

    let texts = vec![
        "Rust 是一门系统编程语言",
        "Python 是一门解释型语言",
        "Rust 注重内存安全和性能",
        "今天天气很好",
    ];

    let embeddings = EmbeddingsBuilder::new(model.clone())
        .documents(texts.clone())?
        .build()
        .await?;

    // 计算余弦相似度
    fn cosine_similarity(a: &[f64], b: &[f64]) -> f64 {
        let dot: f64 = a.iter().zip(b.iter()).map(|(x, y)| x * y).sum();
        let norm_a: f64 = a.iter().map(|x| x * x).sum::<f64>().sqrt();
        let norm_b: f64 = b.iter().map(|x| x * x).sum::<f64>().sqrt();
        dot / (norm_a * norm_b)
    }

    let query = "Rust 的特点是什么";
    let query_embedding = model.embed_text(query).await?;

    println!("查询: {}\n", query);
    for (i, embedding) in embeddings.iter().enumerate() {
        let similarity = cosine_similarity(&query_embedding.vec, &embedding.vec);
        println!("相似度 {:.4}: {}", similarity, texts[i]);
    }

    Ok(())
}
```

**完成标准：**
- [ ] 理解 Embedding 的概念
- [ ] 观察语义相似的句子得分更高

---

### Day 12-13: RAG 基础

**学习内容：**
- [ ] RAG 是什么（Retrieval Augmented Generation）
- [ ] RAG 的流程：索引 -> 检索 -> 生成
- [ ] Rig 的向量存储

**练习：简单的内存 RAG**
```rust
use rig::providers::openai;
use rig::vector_store::in_memory_store::InMemoryVectorStore;
use rig::embeddings::EmbeddingsBuilder;
use rig::vector_store::VectorStoreIndex;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = openai::Client::from_env();
    let embedding_model = client.embedding_model(openai::TEXT_EMBEDDING_3_SMALL);

    // 知识库文档
    let documents = vec![
        "Rust 的所有权系统确保内存安全，无需垃圾回收器。",
        "Rust 的借用检查器在编译时防止数据竞争。",
        "Cargo 是 Rust 的包管理器和构建工具。",
        "Rust 的 trait 类似于其他语言的接口。",
        "Rust 可以编译为 WebAssembly，在浏览器中运行。",
    ];

    // 创建嵌入
    let embeddings = EmbeddingsBuilder::new(embedding_model.clone())
        .documents(documents.clone())?
        .build()
        .await?;

    // 创建向量存储
    let vector_store = InMemoryVectorStore::from_documents(embeddings);
    let index = vector_store.index(embedding_model);

    // 检索相关文档
    let query = "Rust 如何保证内存安全？";
    let results = index.top_n::<String>(query, 2).await?;

    println!("查询: {}\n", query);
    println!("相关文档:");
    for (score, _, doc) in &results {
        println!("  [{:.4}] {}", score, doc);
    }

    // 构建 RAG prompt
    let context: Vec<_> = results.iter().map(|(_, _, doc)| doc.as_str()).collect();
    let rag_prompt = format!(
        "根据以下信息回答问题。\n\n信息：\n{}\n\n问题：{}",
        context.join("\n"),
        query
    );

    let agent = client
        .agent(openai::GPT_4O_MINI)
        .build();

    let response = agent.prompt(&rag_prompt).await?;
    println!("\n回答: {}", response);

    Ok(())
}
```

**完成标准：**
- [ ] 理解 RAG 的完整流程
- [ ] 能解释为什么 RAG 比直接问 LLM 效果更好

---

### Day 14: 第二周回顾和巩固

**任务：**
- [ ] 回顾本周代码，确保每个概念都理解
- [ ] 整理笔记
- [ ] 解决遗留问题

**自测问题：**
1. Agent 和 Completion 的区别是什么？
2. Tool trait 需要实现哪些关联类型？
3. Embedding 向量的维度代表什么？
4. RAG 中的 "R"、"A"、"G" 分别是什么？

---

## 第三周：项目开发 - PDF 问答助手

### Day 15: 项目架构设计

**项目目标：**
构建一个命令行工具，可以：
1. 加载 PDF 文件
2. 将内容切分并建立索引
3. 回答关于 PDF 内容的问题

**技术栈：**
- Rig（LLM 和 Embedding）
- pdf-extract（PDF 解析）
- 内存向量存储

**项目结构：**
```
pdf-qa/
├── Cargo.toml
├── src/
│   ├── main.rs           # CLI 入口
│   ├── lib.rs            # 库入口
│   ├── pdf/
│   │   ├── mod.rs
│   │   └── parser.rs     # PDF 解析
│   ├── index/
│   │   ├── mod.rs
│   │   ├── chunker.rs    # 文本切分
│   │   └── store.rs      # 向量存储
│   ├── agent/
│   │   ├── mod.rs
│   │   └── qa.rs         # 问答 Agent
│   └── error.rs          # 错误类型
└── tests/
    └── integration.rs
```

**依赖：**
```toml
[dependencies]
rig-core = "0.9"
tokio = { version = "1", features = ["full"] }
pdf-extract = "0.7"
clap = { version = "4", features = ["derive"] }
thiserror = "2"
anyhow = "1"
serde = { version = "1", features = ["derive"] }
serde_json = "1"
```

**完成标准：**
- [ ] 项目结构创建完成
- [ ] 所有依赖安装成功

---

### Day 16: PDF 解析模块

**任务：** 实现 PDF 文本提取

```rust
// src/pdf/parser.rs
use std::path::Path;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum PdfError {
    #[error("无法读取文件: {0}")]
    IoError(#[from] std::io::Error),

    #[error("PDF 解析失败: {0}")]
    ParseError(String),
}

pub struct PdfDocument {
    pub path: String,
    pub content: String,
    pub page_count: usize,
}

impl PdfDocument {
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self, PdfError> {
        let path_str = path.as_ref().display().to_string();
        let bytes = std::fs::read(&path)?;

        let content = pdf_extract::extract_text_from_mem(&bytes)
            .map_err(|e| PdfError::ParseError(e.to_string()))?;

        // 估算页数（简化版）
        let page_count = content.matches("\x0c").count().max(1);

        Ok(Self {
            path: path_str,
            content,
            page_count,
        })
    }

    pub fn preview(&self, chars: usize) -> &str {
        if self.content.len() <= chars {
            &self.content
        } else {
            &self.content[..chars]
        }
    }
}
```

**测试：**
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_pdf() {
        // 下载一个测试 PDF 或使用本地文件
        let doc = PdfDocument::load("test.pdf").unwrap();
        assert!(!doc.content.is_empty());
    }
}
```

**完成标准：**
- [ ] 能成功提取 PDF 文本
- [ ] 错误处理完善

---

### Day 17: 文本切分模块

**任务：** 将长文本切分为适合 Embedding 的小块

```rust
// src/index/chunker.rs

#[derive(Debug, Clone)]
pub struct Chunk {
    pub id: usize,
    pub content: String,
    pub start_char: usize,
    pub end_char: usize,
}

pub struct ChunkerConfig {
    pub chunk_size: usize,      // 每块的目标字符数
    pub chunk_overlap: usize,   // 块之间的重叠字符数
}

impl Default for ChunkerConfig {
    fn default() -> Self {
        Self {
            chunk_size: 500,
            chunk_overlap: 50,
        }
    }
}

pub struct Chunker {
    config: ChunkerConfig,
}

impl Chunker {
    pub fn new(config: ChunkerConfig) -> Self {
        Self { config }
    }

    pub fn chunk(&self, text: &str) -> Vec<Chunk> {
        let mut chunks = Vec::new();
        let mut start = 0;
        let mut id = 0;

        while start < text.len() {
            let end = (start + self.config.chunk_size).min(text.len());

            // 尝试在句号或换行处断开
            let actual_end = self.find_break_point(text, start, end);

            let content = text[start..actual_end].trim().to_string();

            if !content.is_empty() {
                chunks.push(Chunk {
                    id,
                    content,
                    start_char: start,
                    end_char: actual_end,
                });
                id += 1;
            }

            // 下一块的起始位置（考虑重叠）
            start = if actual_end >= text.len() {
                actual_end
            } else {
                actual_end.saturating_sub(self.config.chunk_overlap)
            };
        }

        chunks
    }

    fn find_break_point(&self, text: &str, start: usize, end: usize) -> usize {
        if end >= text.len() {
            return text.len();
        }

        // 从 end 往前找句号或换行
        let search_region = &text[start..end];

        for (i, c) in search_region.char_indices().rev() {
            if c == '。' || c == '.' || c == '\n' {
                return start + i + c.len_utf8();
            }
        }

        // 没找到好的断点，就在空格处断开
        for (i, c) in search_region.char_indices().rev() {
            if c.is_whitespace() {
                return start + i;
            }
        }

        end
    }
}
```

**完成标准：**
- [ ] 能正确切分中英文文本
- [ ] 重叠机制工作正常

---

### Day 18: 向量索引模块

**任务：** 集成 Rig 向量存储

```rust
// src/index/store.rs
use rig::providers::openai;
use rig::vector_store::in_memory_store::InMemoryVectorStore;
use rig::embeddings::EmbeddingsBuilder;
use rig::vector_store::VectorStoreIndex;
use crate::index::chunker::Chunk;

pub struct DocumentIndex {
    chunks: Vec<Chunk>,
    index: InMemoryVectorStore<openai::EmbeddingModel>,
}

impl DocumentIndex {
    pub async fn build(
        client: &openai::Client,
        chunks: Vec<Chunk>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let embedding_model = client.embedding_model(openai::TEXT_EMBEDDING_3_SMALL);

        let contents: Vec<String> = chunks.iter()
            .map(|c| c.content.clone())
            .collect();

        let embeddings = EmbeddingsBuilder::new(embedding_model)
            .documents(contents)?
            .build()
            .await?;

        let store = InMemoryVectorStore::from_documents(embeddings);

        Ok(Self {
            chunks,
            index: store,
        })
    }

    pub async fn search(
        &self,
        query: &str,
        top_k: usize,
    ) -> Result<Vec<&Chunk>, Box<dyn std::error::Error>> {
        let embedding_model = /* ... */;
        let results = self.index
            .index(embedding_model)
            .top_n::<String>(query, top_k)
            .await?;

        // 根据返回的内容匹配原始 Chunk
        let matched_chunks: Vec<_> = results
            .iter()
            .filter_map(|(_, _, content)| {
                self.chunks.iter().find(|c| &c.content == content)
            })
            .collect();

        Ok(matched_chunks)
    }
}
```

**完成标准：**
- [ ] 索引构建成功
- [ ] 搜索返回相关结果

---

### Day 19: 问答 Agent

**任务：** 实现 RAG 问答逻辑

```rust
// src/agent/qa.rs
use rig::providers::openai;

pub struct QAAgent {
    client: openai::Client,
}

impl QAAgent {
    pub fn new(client: openai::Client) -> Self {
        Self { client }
    }

    pub async fn answer(
        &self,
        question: &str,
        context_chunks: &[&str],
    ) -> Result<String, Box<dyn std::error::Error>> {
        let context = context_chunks.join("\n\n---\n\n");

        let system_prompt = r#"你是一个专业的文档问答助手。
根据提供的文档内容回答用户问题。

规则：
1. 只根据提供的内容回答，不要编造信息
2. 如果内容中没有相关信息，请明确告知用户
3. 引用具体内容时，保持准确
4. 回答要简洁明了"#;

        let user_prompt = format!(
            "文档内容：\n{}\n\n问题：{}",
            context, question
        );

        let agent = self.client
            .agent(openai::GPT_4O_MINI)
            .preamble(system_prompt)
            .build();

        let response = agent.prompt(&user_prompt).await?;

        Ok(response)
    }
}
```

**完成标准：**
- [ ] 能根据上下文生成回答
- [ ] 当信息不足时能正确提示

---

### Day 20-21: CLI 整合和测试

**任务：** 完成命令行界面

```rust
// src/main.rs
use clap::{Parser, Subcommand};
use pdf_qa::{PdfDocument, Chunker, ChunkerConfig, DocumentIndex, QAAgent};
use rig::providers::openai;
use std::io::{self, Write};

#[derive(Parser)]
#[command(name = "pdf-qa")]
#[command(about = "PDF 文档问答助手")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// 加载 PDF 并进入问答模式
    Chat {
        /// PDF 文件路径
        #[arg(short, long)]
        file: String,
    },
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Chat { file } => {
            println!("正在加载 PDF: {}", file);
            let doc = PdfDocument::load(&file)?;
            println!("已加载 {} 页，共 {} 字符\n", doc.page_count, doc.content.len());

            println!("正在建立索引...");
            let chunker = Chunker::new(ChunkerConfig::default());
            let chunks = chunker.chunk(&doc.content);
            println!("已切分为 {} 个文本块\n", chunks.len());

            let client = openai::Client::from_env();
            let index = DocumentIndex::build(&client, chunks).await?;
            let agent = QAAgent::new(client);

            println!("索引建立完成！开始问答（输入 quit 退出）\n");

            loop {
                print!("问题: ");
                io::stdout().flush()?;

                let mut input = String::new();
                io::stdin().read_line(&mut input)?;
                let input = input.trim();

                if input == "quit" || input == "exit" {
                    break;
                }

                if input.is_empty() {
                    continue;
                }

                // 检索相关内容
                let relevant_chunks = index.search(input, 3).await?;
                let context: Vec<_> = relevant_chunks.iter()
                    .map(|c| c.content.as_str())
                    .collect();

                // 生成回答
                let answer = agent.answer(input, &context).await?;
                println!("\n回答: {}\n", answer);
            }
        }
    }

    Ok(())
}
```

**测试清单：**
- [ ] 加载小型 PDF（< 10 页）
- [ ] 加载中型 PDF（10-50 页）
- [ ] 测试中文 PDF
- [ ] 测试英文 PDF
- [ ] 测试错误情况（文件不存在、格式错误）

---

## 第四周：完善和部署

### Day 22-23: 功能增强

**可选功能（选择 2-3 个实现）：**

1. **持久化索引**
   - 将向量索引保存到文件
   - 下次加载相同 PDF 时跳过索引步骤

2. **流式输出**
   - 使用 Rig 的 streaming API
   - 实时显示 AI 回答

3. **多文件支持**
   - 同时加载多个 PDF
   - 跨文件搜索和问答

4. **引用来源**
   - 在回答中标注信息来源的页码
   - 显示相关度分数

5. **Web 界面**
   - 使用 Axum 或 Actix-web
   - 简单的上传和问答 UI

---

### Day 24-25: 代码质量

**任务：**

1. **添加文档注释**
```rust
/// PDF 文档问答助手的核心索引结构
///
/// # Example
/// ```
/// let index = DocumentIndex::build(&client, chunks).await?;
/// let results = index.search("什么是所有权？", 3).await?;
/// ```
pub struct DocumentIndex { ... }
```

2. **添加单元测试**
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chunker_basic() {
        let text = "这是第一句。这是第二句。这是第三句。";
        let chunker = Chunker::new(ChunkerConfig {
            chunk_size: 10,
            chunk_overlap: 2,
        });
        let chunks = chunker.chunk(text);
        assert!(chunks.len() > 1);
    }
}
```

3. **运行 Clippy**
```bash
cargo clippy -- -D warnings
```

4. **格式化代码**
```bash
cargo fmt
```

---

### Day 26-27: README 和演示

**README.md 模板：**
```markdown
# PDF-QA: 智能文档问答助手

基于 Rust 和 Rig 框架构建的 RAG 应用，可以对 PDF 文档进行智能问答。

## 功能特点

- 🔍 语义搜索：使用向量嵌入进行智能检索
- 💬 自然语言问答：基于文档内容回答问题
- ⚡ 高性能：Rust 实现，内存安全且高效
- 📄 PDF 支持：自动解析和切分 PDF 文档

## 技术栈

- **语言**: Rust
- **LLM 框架**: Rig
- **模型**: OpenAI GPT-4o-mini / text-embedding-3-small
- **PDF 解析**: pdf-extract

## 安装

\`\`\`bash
git clone https://github.com/yourusername/pdf-qa
cd pdf-qa
cargo build --release
\`\`\`

## 使用

\`\`\`bash
export OPENAI_API_KEY="your-api-key"
./target/release/pdf-qa chat --file document.pdf
\`\`\`

## 演示

[插入 GIF 或截图]

## 架构

\`\`\`
PDF → 解析 → 切分 → Embedding → 向量索引
                                    ↓
用户问题 → Embedding → 相似度搜索 → 检索结果
                                    ↓
                        LLM 生成回答 ← 上下文
\`\`\`

## License

MIT
```

**制作演示：**
- 使用 `asciinema` 录制终端演示
- 或使用 `terminalizer` 生成 GIF

---

### Day 28: 发布

**任务清单：**

1. **GitHub 仓库**
   - [ ] 创建仓库
   - [ ] 推送代码
   - [ ] 添加 .gitignore
   - [ ] 添加 LICENSE

2. **完善 README**
   - [ ] 添加 badges（build status、license）
   - [ ] 确保安装步骤可行

3. **LinkedIn / 博客**
   - [ ] 写一篇项目介绍（可选）
   - [ ] 分享到技术社区

---

## 学习资源汇总

### 官方文档
- Rust Book: https://doc.rust-lang.org/book/
- Async Book: https://rust-lang.github.io/async-book/
- Tokio Tutorial: https://tokio.rs/tokio/tutorial
- Rig Docs: https://docs.rs/rig-core/

### 推荐阅读
- Rust by Example: https://doc.rust-lang.org/rust-by-example/
- Zero to Production in Rust: https://www.zero2prod.com/

### 社区
- Rust 中文社区: https://rustcc.cn/
- Rust 官方 Discord
- r/rust on Reddit

---

## 进度追踪

| 周 | 主题 | 状态 |
|----|------|------|
| 1 | 异步 Rust | ⬜ 未开始 |
| 2 | Rig 基础 | ⬜ 未开始 |
| 3 | 项目开发 | ⬜ 未开始 |
| 4 | 完善部署 | ⬜ 未开始 |

---

> 💡 提示：每天学习后，在此文档中勾选完成的任务，便于追踪进度。
