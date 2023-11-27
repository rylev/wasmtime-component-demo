use pulldown_cmark::{html, Parser};

wit_bindgen::generate!({
    path: "wit",
    exports: {
        world: MarkdownImpl
    }
});

struct MarkdownImpl;

impl Guest for MarkdownImpl {
    fn render() -> String {
        let input = readline();
        let parser = Parser::new(&input);
        let mut html_output = String::new();
        html::push_html(&mut html_output, parser);
        html_output
    }
}
