use crate::models::markdown::MarkdownId;
use graphql_client::GraphQLQuery;
use serde::{Deserialize, Serialize};
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response};

#[derive(Debug, Serialize, Deserialize)]
struct GraphQLResponse {
    data: GraphQLData,
}

#[derive(Debug, Serialize, Deserialize)]
struct GraphQLData {
    deleteMarkdown: MarkdownId,
}

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/delete_markdown.graphql",
    response_derives = "Debug"
)]
struct DeleteMarkdown;

pub async fn fetch_delete_markdown(id: String) -> Result<String, JsValue> {
    let build_query = DeleteMarkdown::build_query(delete_markdown::Variables { id: id });
    let query = serde_json::json!(build_query);
    let mut opts = RequestInit::new();
    opts.method("POST");
    opts.body(Some(&JsValue::from_str(query.to_string().as_str())));
    opts.mode(RequestMode::Cors);
    let url = String::from("http://localhost:8081/graphql");
    let request = Request::new_with_str_and_init(url.as_str(), &opts)?;
    request.headers().set("Content-Type", "application/json")?;

    let window = web_sys::window().unwrap();
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;
    let resp: Response = resp_value.dyn_into().unwrap();
    let json = JsFuture::from(resp.json()?).await?;

    let response_data: GraphQLResponse = json.into_serde().unwrap();
    Ok(response_data.data.deleteMarkdown.id)
}
