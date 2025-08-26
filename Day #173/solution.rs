// Flatten nested dictionary, namespacing keys with '.'. Recursive DFS preserving insertion order.
// Time O(total keys), Space O(total keys).

// Value is either an integer leaf or an ordered nested map.
enum Value {
    Leaf(i64),
    Map(Vec<(String, Value)>),
}

fn flatten(node: &Value, prefix: &str, out: &mut Vec<(String, i64)>) {
    match node {
        Value::Leaf(v) => out.push((prefix.to_string(), *v)),
        Value::Map(children) => {
            for (k, child) in children {
                let key = if prefix.is_empty() {
                    k.clone()
                } else {
                    format!("{}.{}", prefix, k)
                };
                flatten(child, &key, out);
            }
        }
    }
}

fn main() {
    // {"key":3,"foo":{"a":5,"bar":{"baz":8}}}
    let root = Value::Map(vec![
        ("key".to_string(), Value::Leaf(3)),
        (
            "foo".to_string(),
            Value::Map(vec![
                ("a".to_string(), Value::Leaf(5)),
                (
                    "bar".to_string(),
                    Value::Map(vec![("baz".to_string(), Value::Leaf(8))]),
                ),
            ]),
        ),
    ]);

    let mut out: Vec<(String, i64)> = Vec::new();
    flatten(&root, "", &mut out);
    for (k, v) in &out {
        println!("{}: {}", k, v);
    }
}
