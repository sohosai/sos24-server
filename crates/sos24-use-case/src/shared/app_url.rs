use sos24_domain::entity::{form::FormId, news::NewsId, project::ProjectId};

use super::context::ContextProvider;

pub fn form(ctx: &impl ContextProvider, form_id: FormId) -> String {
    format!("{}/forms/{}", ctx.config().app_url, form_id.value())
}

pub fn news(ctx: &impl ContextProvider, news_id: NewsId) -> String {
    format!("{}/news/{}", ctx.config().app_url, news_id.value())
}

pub fn committee_form(ctx: &impl ContextProvider, form_id: FormId) -> String {
    format!(
        "{}/committee/forms/{}",
        ctx.config().app_url,
        form_id.value()
    )
}

pub fn committee_news(ctx: &impl ContextProvider, news_id: NewsId) -> String {
    format!(
        "{}/committee/news/{}",
        ctx.config().app_url,
        news_id.value()
    )
}

pub fn committee_project(ctx: &impl ContextProvider, project_id: ProjectId) -> String {
    format!(
        "{}/committee/projects/{}",
        ctx.config().app_url,
        project_id.value()
    )
}
