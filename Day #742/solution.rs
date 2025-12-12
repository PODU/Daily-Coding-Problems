// Flatten a nested dictionary, joining keys with '.' via DFS.
// Ordered Vec of pairs preserves insertion order.
// Time: O(total keys), Space: O(depth) recursion.

enum Value {
    Leaf(i32),
    Obj(Vec<(String, Value)>),
}

fn flatten(prefix: &str, v: &Value, out: &mut Vec<(String, i32)>) {
    if let Value::Obj(pairs) = v {
        for (k, val) in pairs {
            let key = if prefix.is_empty() {
                k.clone()
            } else {
                format!("{}.{}", prefix, k)
            };
            match val {
                Value::Leaf(n) => out.push((key, *n)),
                Value::Obj(_) => flatten(&key, val, out),
            }
        }
    }
}

fn main() {
    let root = Value::Obj(vec![
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
    flatten("", &root, &mut out);
    let parts: Vec<String> = out.iter().map(|(k, v)| format!("'{}': {}", k, v)).collect();
    println!("{{{}}}", parts.join(", "));
    // {'key': 3, 'foo.a': 5, 'foo.bar.baz': 8}
}
