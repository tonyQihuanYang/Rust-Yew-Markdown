use graphql_client::GraphQLQuery;
use serde::{Deserialize, Serialize};
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response};

use crate::models::markdown::Markdown;

#[derive(Debug, Serialize, Deserialize)]
pub struct MyResultNew {
    data: MyResult,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MyResult {
    allMarkdowns: Vec<Markdown>,
}

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./src/graphql_requests/graphql/schema.graphql",
    query_path = "./src/graphql_requests/graphql/all_markdowns.graphql",
    response_derives = "Debug"
)]
struct AllMarkdowns;

pub async fn fetch_markdowns() -> Result<Vec<Markdown>, JsValue> {
    let build_query = AllMarkdowns::build_query(all_markdowns::Variables {});
    let query = serde_json::json!(build_query);
    let mut opts = RequestInit::new();
    opts.method("POST");
    opts.body(Some(&JsValue::from_str(query.to_string().as_str())));
    opts.mode(RequestMode::Cors);
    let url = String::from("https://apps.gummui.com/markdown-api/graphql");
    let request = Request::new_with_str_and_init(url.as_str(), &opts)?;
    request.headers().set("Content-Type", "application/json")?;

    let window = web_sys::window().unwrap();
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;
    let resp: Response = resp_value.dyn_into().unwrap();
    let json = JsFuture::from(resp.json()?).await?;
    let response_data: MyResultNew = json.into_serde().unwrap();

    Ok(response_data.data.allMarkdowns)
}
