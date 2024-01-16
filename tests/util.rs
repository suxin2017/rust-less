#[cfg(feature = "serde")]
macro_rules! test_ast {
  ($code:expr) => {
    let stylesheet = StyleSheet::parse($code, ParserOptions::default()).unwrap();
    insta::assert_yaml_snapshot!(stylesheet);
  };
}
