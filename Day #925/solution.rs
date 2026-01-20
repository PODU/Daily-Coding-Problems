// Flatten nested dict, namespacing keys with '.'. Recurse; on nested map prepend
// parentKey + '.'. Insertion order kept via Vec of pairs.
// Time O(total entries), Space O(depth).

enum Value {
    Int(i64),
    Dict(Vec<(String, Value)>),
}

fn flatten(node: &[(String, Value)], prefix: &str, out: &mut Vec<(String, i64)>) {
    for (k, v) in node {
        let key = format!("{}{}", prefix, k);
        match v {
            Value::Dict(children) => flatten(children, &format!("{}.", key), out),
            Value::Int(n) => out.push((key, *n)),
        }
    }
}

fn main() {
    // {"key": 3, "foo": {"a": 5, "bar": {"baz": 8}}}
    let data: Vec<(String, Value)> = vec![
        ("key".to_string(), Value::Int(3)),
        (
            "foo".to_string(),
            Value::Dict(vec![
                ("a".to_string(), Value::Int(5)),
                (
                    "bar".to_string(),
                    Value::Dict(vec![("baz".to_string(), Value::Int(8))]),
                ),
            ]),
        ),
    ];

    let mut out: Vec<(String, i64)> = Vec::new();
    flatten(&data, "", &mut out);

    let parts: Vec<String> = out
        .iter()
        .map(|(k, v)| format!("\"{}\": {}", k, v))
        .collect();
    println!("{{{}}}", parts.join(", "));
}
