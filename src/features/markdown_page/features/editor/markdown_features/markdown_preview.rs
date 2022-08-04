use gloo_utils::document;
use pulldown_cmark::{html, Options, Parser};
use web_sys::{Element, Node};
use yew::{function_component, Html, Properties};

#[derive(PartialEq, Clone, Properties)]
pub struct HeaderInputProps {
    pub data: String,
}

#[function_component(MarkdownPrview)]
pub fn markdown_preview(props: &HeaderInputProps) -> Html {
    let markdown_html: String = parse_to_html(&props.data);
    let div: Element = document().create_element("div").unwrap();
    div.set_class_name("md-preview-wrapper p-2");
    div.set_inner_html(&markdown_html);
    let node: Node = div.into();
    Html::VRef(node)
}

fn parse_to_html(markdown_input: &String) -> String {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    let parser = Parser::new_ext(markdown_input, options);

    let mut html_output: String = String::with_capacity(markdown_input.len() * 3 / 2);
    html::push_html(&mut html_output, parser);
    return html_output;
}
