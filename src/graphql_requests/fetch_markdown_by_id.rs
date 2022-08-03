use graphql_client::GraphQLQuery;
use serde::{Deserialize, Serialize};
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response};

use crate::models::markdown::Markdown;
use crate::API_URL;

#[derive(Debug, Serialize, Deserialize)]
struct GraphQLResponse {
    data: GraphQLData,
}

#[derive(Debug, Serialize, Deserialize)]
struct GraphQLData {
    markdownById: Markdown,
}

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./src/graphql_requests/graphql/schema.graphql",
    query_path = "./src/graphql_requests/graphql/markdown_by_id.graphql",
    response_derives = "Debug"
)]
struct MarkdownById;

pub async fn fetch_markdown_by_id(id: String) -> Result<Markdown, JsValue> {
    let build_query = MarkdownById::build_query(markdown_by_id::Variables { id: id });
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

    let branch_info: GraphQLResponse = json.into_serde().unwrap();
    Ok(branch_info.data.markdownById)
}
