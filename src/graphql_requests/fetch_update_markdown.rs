use crate::{
    models::markdown::{Markdown, MarkdownUpdated},
    API_URL,
};
use graphql_client::GraphQLQuery;
use serde::{Deserialize, Serialize};
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response};

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./src/graphql_requests/graphql/schema.graphql",
    query_path = "./src/graphql_requests/graphql/update_markdown.graphql",
    response_derives = "Debug"
)]
struct UpdateMarkdown;

pub struct MarkdownUpdateInput {
    pub id: String,
    pub version: i64,
    pub title: Option<String>,
    pub context: Option<String>,
}

#[derive(PartialEq, Debug, Serialize, Deserialize, Clone)]
pub struct UpdateMarkdownResponse {
    pub updateMarkdown: MarkdownUpdated,
}

#[derive(PartialEq, Debug, Serialize, Deserialize, Clone)]
pub struct MarkdownUpdateResponse {
    pub data: UpdateMarkdownResponse,
}

pub async fn fetch_update_markdown(
    updated_markdown: MarkdownUpdateInput,
) -> Result<MarkdownUpdated, String> {
    let error = String::from("ERROR");
    let json = match request_update_markdown(updated_markdown).await {
        Ok(result) => result,
        Err(_) => return Err(error),
    };

    let updated_markdown: MarkdownUpdateResponse = match json.into_serde() {
        Ok(markdown) => markdown,
        Err(_) => return Err(error),
    };
    Ok(updated_markdown.data.updateMarkdown)
}

async fn request_update_markdown(
    updated_markdown: MarkdownUpdateInput,
) -> Result<JsValue, JsValue> {
    let build_query = UpdateMarkdown::build_query(update_markdown::Variables {
        new_markdown: update_markdown::MarkdownUpdateInput {
            id: updated_markdown.id,
            version: updated_markdown.version,
            title: Some(updated_markdown.title).unwrap_or(None),
            context: Some(updated_markdown.context).unwrap_or(None),
        },
    });

    let query = serde_json::json!(build_query);
    let mut opts = RequestInit::new();
    opts.method("POST");
    opts.body(Some(&JsValue::from_str(query.to_string().as_str())));
    opts.mode(RequestMode::Cors);
    let request = Request::new_with_str_and_init(API_URL, &opts)?;
    request.headers().set("Content-Type", "application/json")?;

    let window = web_sys::window().unwrap();
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;
    let resp: Response = resp_value.dyn_into().unwrap();
    let json = JsFuture::from(resp.json()?).await?;
    return Ok(json);
}
