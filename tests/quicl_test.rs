use lightningcss::{stylesheet::{ParserOptions, StyleSheet, PrinterOptions}, targets::{Targets, Browsers, Features}};

#[test]
fn quick_test() {
    let code = r#"
@font-face {
@a:123 456;
  font-family: "Trickster";
  src:
    local("Trickster"),
    url("trickster-COLRv1.otf") format("opentype") tech(color-COLRv1),
    url("trickster-outline.otf") format("opentype"),
    url("trickster-outline.woff") format("woff");
}
  "#;
    let stylesheet = StyleSheet::parse(code, ParserOptions::default()).unwrap();
    dbg!(&stylesheet);
    let mut c = PrinterOptions::default();
    c.targets = Targets {
        browsers: Some(Browsers {
            chrome: Some(50 << 16),
            ..Browsers::default()
        }),

        include: Features::Nesting,
        exclude: Features::empty(),
    };
    let r = stylesheet.to_css(c).unwrap();
    println!("output: \n{}", r.code)
}
