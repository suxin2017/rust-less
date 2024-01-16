use insta::assert_debug_snapshot;
use lightningcss::{
  stylesheet::{ParserOptions, PrinterOptions, StyleSheet},
  targets::{Browsers, Features, Targets},
};

#[test]
fn quick_test() {
  let code = r#"
a {
width: 1 + 2 * 3 + 4;
}
  "#;
  let stylesheet = StyleSheet::parse(code, ParserOptions::default()).unwrap();
  dbg!(&stylesheet);
  // let mut c = PrinterOptions::default();
  // c.targets = Targets {
  //   browsers: Some(Browsers {
  //     chrome: Some(50 << 16),
  //     ..Browsers::default()
  //   }),
  //
  //   include: Features::Nesting,
  //   exclude: Features::empty(),
  // };
  // let r = stylesheet.to_css(c).unwrap();
  // println!("output: \n{}", r.code)
}
