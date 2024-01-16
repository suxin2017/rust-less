#[cfg(feature = "serde")]
use lightningcss::stylesheet::{ParserOptions, StyleSheet};

#[cfg(feature = "serde")]
macro_rules! test_ast {
  ($code:expr) => {
    let stylesheet = StyleSheet::parse($code, ParserOptions::default()).unwrap();
    insta::assert_yaml_snapshot!(stylesheet);
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

#[cfg(feature = "serde")]
#[test]
fn test_at_rule_media() {
  test_ast!(
    r#"
@a: 123;
@media screen and (min-width: 900px) {
  @b: 456;
  article {
    @c: 789;
    padding: 1rem 3rem;
  }
}
  "#
  );
}

#[cfg(feature = "serde")]
#[test]
fn test_at_rule_keyframes() {
  test_ast!(
    r#"
@a: 123;
@keyframes slidein {
    @b: 456;
    from {
        @c: 789;
        transform: translateX(0%);
    }

    to {
        transform: translateX(100%);
    }
}
  "#
  );
}

#[cfg(feature = "serde")]
#[test]
fn test_at_rule_font_face() {
  test_ast!(
    r#"
@a: 123;
@font-face {
    @b: 456;
  font-family: "MyHelvetica";
  src: local("Helvetica Neue Bold"), local("HelveticaNeue-Bold"),
    url("MgOpenModernaBold.ttf");
  font-weight: bold;
}
  "#
  );
}

#[cfg(feature = "serde")]
#[test]
fn test_at_rule_font_palette_values() {
  test_ast!(
    r#"
@a: 123;
@font-palette-values --identifier {
@b: 123;
font-family: Bixa;
}
  "#
  );
}
#[cfg(feature = "serde")]
#[test]
fn test_at_rule_page() {
  test_ast!(
    r#"
@a: 123;
@page {
  @b: 123;
}
@page {
  @c: 123;
  @top-right {
    @d: 123;
  }
}
  "#
  );
}

#[cfg(feature = "serde")]
#[test]
fn test_at_supports() {
  test_ast!(
    r#"
@a: 123;
@supports (display: flex) {
  @b: 123;
  .flex-container {
    @c: 123;
    display: flex;
  }
}
  "#
  );
}
#[cfg(feature = "serde")]
#[test]
fn test_at_counter_style() {
  test_ast!(
    r#"
    @a: 123;
@counter-style thumbs {
@b:456;
  system: cyclic;
  symbols: "\1F44D";
  suffix: " ";
}
  "#
  );
}

#[cfg(feature = "serde")]
#[test]
fn test_nest_rule() {
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

#[cfg(feature = "serde")]
#[test]
fn test_at_rule_layer() {
  test_ast!(
    r#"
@layer utilities {
  @a: 123;
}

@layer utilities {
  @a: 123;
  p {
    @b: 123;
  }
}
  "#
  );
}

#[cfg(feature = "serde")]
#[test]
fn test_at_rule_property() {
  test_ast!(
    r#"
    @a:123;
@property --item-size {
@b:345;
syntax: "<percentage>";
inherits: true;
initial-value: 40%;
}
  "#
  );
}

#[cfg(feature = "serde")]
#[test]
fn test_at_rule_container() {
  test_ast!(
    r#"
    @a:123;
@container (width > 400px) {
@b: 123;
  h2 {
  @c: 345;
    font-size: 1.5em;
  }
}
  "#
  );
}

#[cfg(feature = "serde")]
#[test]
fn test_at_rule_scope() {
  test_ast!(
    r#"
    @a:123;
@scope (.article-body) to (figure) {
 @b: 123;
  h2 {
  @c: 345;
    font-size: 1.5em;
  }
}
  "#
  );
}

#[cfg(feature = "serde")]
#[test]
fn test_at_rule_starting_style() {
  test_ast!(
    r#"
    @a:123;
@starting-style {
 @b:123;
  [popover]:popover-open {
   @c:123;
    opacity: 0;
    transform: scaleX(0);
  }
}
  "#
  );
}
