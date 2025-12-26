use reqwest::header::{HeaderMap, HeaderValue, COOKIE, REFERER};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::fs;
use std::path::Path;

#[derive(Debug, Serialize)]
struct GraphQLRequest {
    query: String,
    variables: serde_json::Value,
}

#[derive(Debug, Deserialize)]
struct GraphQLResponse<T> {
    data: Option<T>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct RecentSubmissionsData {
    recent_submission_list: Option<Vec<Submission>>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Submission {
    id: String,
    title: String,
    title_slug: String,
    lang: String,
    status_display: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct SubmissionDetailsData {
    submission_details: Option<SubmissionDetails>,
}

#[derive(Debug, Deserialize)]
struct SubmissionDetails {
    code: String,
}

#[derive(Debug, Deserialize)]
struct QuestionData {
    question: Option<Question>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Question {
    question_frontend_id: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();

    let session = std::env::var("LEETCODE_SESSION")
        .expect("LEETCODE_SESSION not set in .env");
    let csrf = std::env::var("CSRF_TOKEN")
        .expect("CSRF_TOKEN not set in .env");
    let username = std::env::var("LEETCODE_USERNAME")
        .expect("LEETCODE_USERNAME not set in .env");

    let client = reqwest::Client::new();

    let mut headers = HeaderMap::new();
    headers.insert(
        COOKIE,
        HeaderValue::from_str(&format!("LEETCODE_SESSION={}; csrftoken={}", session, csrf))?,
    );
    headers.insert("x-csrftoken", HeaderValue::from_str(&csrf)?);
    headers.insert(REFERER, HeaderValue::from_static("https://leetcode.com"));

    println!("Fetching recent submissions for {}...", username);

    // Fetch recent submissions
    let query = r#"
        query recentSubmissions($username: String!, $limit: Int!) {
            recentSubmissionList(username: $username, limit: $limit) {
                id
                title
                titleSlug
                lang
                statusDisplay
            }
        }
    "#;

    let request = GraphQLRequest {
        query: query.to_string(),
        variables: serde_json::json!({
            "username": username,
            "limit": 50
        }),
    };

    let response = client
        .post("https://leetcode.com/graphql")
        .headers(headers.clone())
        .json(&request)
        .send()
        .await?;

    let result: GraphQLResponse<RecentSubmissionsData> = response.json().await?;

    let submissions = result
        .data
        .and_then(|d| d.recent_submission_list)
        .unwrap_or_default();

    // Filter for accepted Rust submissions
    let rust_accepted: Vec<_> = submissions
        .into_iter()
        .filter(|s| s.lang == "rust" && s.status_display == "Accepted")
        .collect();

    println!("Found {} accepted Rust submissions", rust_accepted.len());

    // Read existing modules from lib.rs
    let lib_path = Path::new("src/lib.rs");
    let lib_content = fs::read_to_string(lib_path)?;
    let mut existing_modules: HashSet<String> = lib_content
        .lines()
        .filter_map(|line| {
            if line.starts_with("pub mod ") {
                Some(line.replace("pub mod ", "").replace(";", "").trim().to_string())
            } else {
                None
            }
        })
        .collect();

    let mut new_modules = Vec::new();

    for submission in rust_accepted {
        println!("  Fetching code for: {}", submission.title);

        // Fetch problem number
        let question_query = r#"
            query questionData($titleSlug: String!) {
                question(titleSlug: $titleSlug) {
                    questionFrontendId
                }
            }
        "#;

        let question_request = GraphQLRequest {
            query: question_query.to_string(),
            variables: serde_json::json!({
                "titleSlug": submission.title_slug
            }),
        };

        let question_response = client
            .post("https://leetcode.com/graphql")
            .headers(headers.clone())
            .json(&question_request)
            .send()
            .await?;

        let question_result: GraphQLResponse<QuestionData> = question_response.json().await?;
        let problem_id = question_result
            .data
            .and_then(|d| d.question)
            .map(|q| q.question_frontend_id)
            .unwrap_or_else(|| "0".to_string());

        // Create module name: p{number}_{slug} (e.g., p704_binary_search)
        // Prefix with 'p' because Rust identifiers can't start with numbers
        let module_name = format!("p{}_{}", problem_id, submission.title_slug.replace("-", "_"));

        if existing_modules.contains(&module_name) {
            println!("  Skipping {} (already exists)", module_name);
            continue;
        }

        // Fetch submission details
        let details_query = r#"
            query submissionDetails($submissionId: Int!) {
                submissionDetails(submissionId: $submissionId) {
                    code
                }
            }
        "#;

        let details_request = GraphQLRequest {
            query: details_query.to_string(),
            variables: serde_json::json!({
                "submissionId": submission.id.parse::<i64>().unwrap_or(0)
            }),
        };

        let details_response = client
            .post("https://leetcode.com/graphql")
            .headers(headers.clone())
            .json(&details_request)
            .send()
            .await?;

        let details_result: GraphQLResponse<SubmissionDetailsData> =
            details_response.json().await?;

        if let Some(details) = details_result.data.and_then(|d| d.submission_details) {
            // Process code: remove "impl Solution" wrapper, we'll add our own
            let code = details.code;

            // Write the file
            let file_path = format!("src/{}.rs", module_name);
            let file_content = format!(
                "// LeetCode: {}\n// https://leetcode.com/problems/{}/\n\nuse crate::Solution;\n\n{}",
                submission.title,
                submission.title_slug,
                code
            );

            fs::write(&file_path, file_content)?;
            println!("    Created: {}", file_path);

            new_modules.push(module_name.clone());
            existing_modules.insert(module_name);
        }
    }

    // Update lib.rs with new modules
    if !new_modules.is_empty() {
        let mut lib_content = fs::read_to_string(lib_path)?;
        for module in &new_modules {
            lib_content.push_str(&format!("pub mod {};\n", module));
        }
        fs::write(lib_path, lib_content)?;
        println!("\nUpdated lib.rs with {} new modules", new_modules.len());
    } else {
        println!("\nNo new modules to add");
    }

    println!("Done!");
    Ok(())
}
