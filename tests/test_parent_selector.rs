#[cfg(feature = "serde")]
use lightningcss::printer::PrinterOptions;
#[cfg(feature = "serde")]
use lightningcss::stylesheet::{ParserOptions, StyleSheet};
#[cfg(feature = "serde")]
use lightningcss::targets::{Browsers, Features, Targets};

#[cfg(feature = "serde")]
macro_rules! test_ast {
  ($code:expr) => {
    let stylesheet = StyleSheet::parse($code, ParserOptions::default()).unwrap();
    let targets = Targets {
      browsers: Some(Browsers {
        chrome: Some(95 << 16),
        ..Browsers::default()
      }),
      include: Features::all(),
      exclude: Features::empty(),
    };
    let res = stylesheet
      .to_css(PrinterOptions {
        targets,
        ..PrinterOptions::default()
      })
      .unwrap();

    insta::assert_snapshot!(res.code);
  };
}

#[cfg(feature = "serde")]
#[test]
fn quick_test() {
  test_ast!(
    r#"
@a: 123;
.a {
  @a: 456;

  .b {
    @c: f;
  }
}
  "#
  );
}
