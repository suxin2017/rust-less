use insta::assert_debug_snapshot;
use lightningcss::{
  stylesheet::{ParserOptions, PrinterOptions, StyleSheet},
  targets::{Browsers, Features, Targets},
};

#[test]
fn quick_test() {
  let code = r#"
h1, h2, h3 {
  a, p {
    &:hover {
      color: red;
    }
  }
}

#all { color: blue; }
#the { color: blue; }
#same { color: blue; }

ul, li, div, q, blockquote, textarea {
  margin: 0;
}

td {
  margin: 0;
  padding: 0;
}

td, input {
  line-height: 1em;
}

a {
  color: red;

  &:hover { color: blue; }

  div & { color: green; }

  p & span { color: yellow; }
}

.foo {
  .bar, .baz {
    & .qux {
      display: block;
    }
    .qux & {
      display: inline;
    }
    .qux& {
      display: inline-block;
    }
    .qux & .biz {
      display: none;
    }
  }
}

.b {
 &.c {
  .a& {
   color: red;
  }
 }
}

.b {
 .c & {
  &.a {
   color: red;
  }
 }
}

.p {
  .foo &.bar {
    color: red;
  }
}

.p {
  .foo&.bar {
    color: red;
  }
}

.foo {
  .foo + & {
    background: amber;
  }
  & + & {
    background: amber;
  }
}

.foo, .bar {
  & + & {
    background: amber;
  }
}



.foo, .bar {
  a, b {
    & > & {
      background: amber;
    }
  }
}

.other ::fnord { color: red }
.other::fnord { color: red }
.other {
  ::bnord {color: red }
  &::bnord {color: red }
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

    include: Features::empty(),
    exclude: Features::empty(),
  };
  let r = stylesheet.to_css(c).unwrap();
  println!("input: {}", code);
  println!("output: \n{}", r.code)
}
