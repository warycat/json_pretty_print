use std::collections::BTreeMap;
use std::fmt;

enum Json {
    Number(i32),
    Str(String),
    Null,
    Arr(Vec<Json>),
    Dict(BTreeMap<String, Json>),
    Bool(bool),
}

impl Json {
    fn pretty(&self, level: usize, all: &mut String) {
        match self {
            Json::Number(x) => {
                all.push_str(&x.to_string());
            }
            Json::Str(s) => {
                all.push('"');
                all.push_str(&s.to_string());
                all.push('"');
            }
            Json::Null => {
                all.push('"');
                all.push_str("Null");
                all.push('"');
            }
            Json::Arr(a) => {
                all.push('[');
                all.push('\n');
                let n = a.len();
                let mut i = 0;
                for x in a {
                    i += 1;
                    for _ in 0..level + 1 {
                        all.push_str("  ")
                    }
                    x.pretty(level + 1, all);
                    if i != n {
                        all.push(',');
                    }
                    all.push('\n');
                }
                for _ in 0..level {
                    all.push_str("  ")
                }
                all.push(']');
            }
            Json::Dict(d) => {
                all.push('{');
                all.push('\n');
                let n = d.len();
                let mut i = 0;
                for (k, v) in d {
                    i += 1;
                    for _ in 0..level + 1 {
                        all.push_str("  ")
                    }
                    all.push_str(&format!("\"{}\": ", k));
                    v.pretty(level + 1, all);
                    if i != n {
                        all.push(',');
                    }
                    all.push('\n');
                }
                for _ in 0..level {
                    all.push_str("  ")
                }
                all.push('}');
            }
            Json::Bool(b) => {
                all.push('"');
                all.push_str(&b.to_string());
                all.push('"');
            }
        };
    }
}

impl std::fmt::Display for Json {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = "".to_string();
        self.pretty(0, &mut s);
        write!(f, "{}", s)
    }
}

const output: &str = r#"{
  "arr": [
    1,
    2,
    {
      "foo": "bar"
    }
  ],
  "num": 1,
  "string": "foo",
  "subobj": {
    "foo": "bar"
  }
}"#;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let mut foo: BTreeMap<String, Json> = BTreeMap::new();
        foo.insert("foo".to_string(), Json::Str("bar".to_string()));
        let arr: Vec<Json> = vec![Json::Number(1), Json::Number(2), Json::Dict(foo)];
        let mut root: BTreeMap<String, Json> = BTreeMap::new();
        root.insert("arr".to_string(), Json::Arr(arr));
        root.insert("num".to_string(), Json::Number(1));
        root.insert("string".to_string(), Json::Str("foo".to_string()));
        let mut subobj: BTreeMap<String, Json> = BTreeMap::new();
        subobj.insert("foo".to_string(), Json::Str("bar".to_string()));
        root.insert("subobj".to_string(), Json::Dict(subobj));
        let obj = Json::Dict(root);
        println!("{}", obj.to_string());
        println!("{}", output);
        assert_eq!(obj.to_string(), output);
    }
}
