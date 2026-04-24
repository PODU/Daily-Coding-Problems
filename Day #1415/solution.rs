// Day 1415: flatten a nested dictionary, namespacing keys with '.'.
// Approach: recursive DFS building prefixed keys. O(total keys) time/space.

enum Value {
    Leaf(i64),
    Obj(Vec<(String, Value)>), // ordered
}

fn flatten(prefix: &str, v: &Value, out: &mut Vec<(String, i64)>) {
    match v {
        Value::Leaf(n) => out.push((prefix.to_string(), *n)),
        Value::Obj(pairs) => {
            for (k, child) in pairs {
                let key = if prefix.is_empty() {
                    k.clone()
                } else {
                    format!("{}.{}", prefix, k)
                };
                flatten(&key, child, out);
            }
        }
    }
}

fn main() {
    let data = Value::Obj(vec![
        ("key".to_string(), Value::Leaf(3)),
        (
            "foo".to_string(),
            Value::Obj(vec![
                ("a".to_string(), Value::Leaf(5)),
                (
                    "bar".to_string(),
                    Value::Obj(vec![("baz".to_string(), Value::Leaf(8))]),
                ),
            ]),
        ),
    ]);
    let mut out = Vec::new();
    flatten("", &data, &mut out);
    for (k, v) in out {
        println!("{}: {}", k, v);
    }
}
