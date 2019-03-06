#![feature(test)]

extern crate pulldown_cmark;
extern crate test;

mod to_html {
    use pulldown_cmark::{Parser, Options, html};
    use std::str::from_utf8;
    use std::io::Read;
    use std::path::Path;
    use std::fs::File;

    fn render_html(text: &str, opts: Options) -> String {
        let mut s = String::with_capacity(text.len() * 3 / 2);
        let p = Parser::new_ext(text, opts);
        html::push_html(&mut s, p);
        s
    }

    #[bench]
    fn crdt_empty_options(b: &mut test::Bencher) {
        let input_bytes = include_bytes!("crdt.md");
        let input = from_utf8(input_bytes).unwrap();
        let mut opts = Options::empty();

        b.iter(|| render_html(&input, opts));
    }
}