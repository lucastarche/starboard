use scraper::{Html, Selector};
use selectors::attr::CaseSensitivity::CaseSensitive;

pub enum ProblemStatus {
    Pending,
    Attempted,
    Completed,
}

pub struct Problem {
    pub task_link: String,
    pub title: String,
    pub status: ProblemStatus,
}

pub async fn query_user_status(user_id: u64) -> anyhow::Result<Vec<Problem>> {
    let url = format!("https://cses.fi/problemset/user/{user_id}/");
    let response = reqwest::get(url).await?;
    let text = response.text().await?;

    let doc = Html::parse_document(&text);
    let problem_selector = Selector::parse("a.task-score").unwrap();
    Ok(doc
        .select(&problem_selector)
        .map(|element| {
            let element = element.value();
            let title = element.attr("title").unwrap_or("Unknown problem");
            let link = element.attr("href").unwrap_or_default();
            let is_full = element.has_class("full", CaseSensitive);
            let is_zero = element.has_class("zero", CaseSensitive);

            let status = if is_full {
                ProblemStatus::Completed
            } else if is_zero {
                ProblemStatus::Attempted
            } else {
                ProblemStatus::Pending
            };

            Problem {
                task_link: link.to_string(),
                title: title.to_string(),
                status,
            }
        })
        .collect())
}
