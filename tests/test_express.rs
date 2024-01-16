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
fn test() {
  test_ast!(
    r#"
a{
width:1;
width: 1+ 2;
width: 1+ 2 * 3;
width: 1+ 2 * 3 +4;
width: 1+ 3 /2 +3;
width: 1+ 2 * (1 + 3);
wdith: 2/(9)+6*9;
width: (8)/(1)/(2)/(2);
width: 7-4+7-4;
width: (7)+(7)+3+(1);
width: 5*(5)+8+(2);
width:1*(9)-(5)*6;
width:(2)+(4)/(5)+(2);
width:1/7/1/2;
width:(6)-(7)*6/(8);
width:(1)-9/3+6;
}
  "#
  );
}
