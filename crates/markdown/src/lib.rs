use pulldown_cmark::{html, Parser};

wit_bindgen::generate!("markdown");

struct MarkdownImpl;
export_markdown!(MarkdownImpl);

impl Markdown for MarkdownImpl {
    fn render(input: String) -> String {
        let parser = Parser::new(&input);
        let mut html_output = String::new();
        html::push_html(&mut html_output, parser);
        html_output
    }
}
