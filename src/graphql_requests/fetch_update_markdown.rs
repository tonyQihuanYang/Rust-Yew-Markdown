use graphql_client::GraphQLQuery;
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response};

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./graphql/update_markdown.graphql",
    response_derives = "Debug"
)]
struct UpdateMarkdown;

pub struct MarkdownUpdateInput {
    pub id: String,
    pub title: Option<String>,
    pub context: Option<String>,
}

pub async fn fetch_update_markdown(new_markdown: MarkdownUpdateInput) -> Result<JsValue, JsValue> {
    let build_query = UpdateMarkdown::build_query(update_markdown::Variables {
        new_markdown: update_markdown::MarkdownUpdateInput {
            id: new_markdown.id,
            title: Some(new_markdown.title).unwrap_or(None),
            context: Some(new_markdown.context).unwrap_or(None),
        },
    });

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
    Ok(json)
}
