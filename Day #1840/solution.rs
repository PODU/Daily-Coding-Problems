// Day 1840: Flatten a nested dictionary, namespacing keys with '.'.
// Recursion over an ordered tree; Time/Space O(total keys).

enum Value {
    Leaf(i64),
    Dict(Vec<(String, Value)>),
}

fn flatten(v: &Value, prefix: &str, out: &mut Vec<(String, i64)>) {
    match v {
        Value::Leaf(n) => out.push((prefix.to_string(), *n)),
        Value::Dict(items) => {
            for (k, child) in items {
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
    let bar = Value::Dict(vec![("baz".into(), Value::Leaf(8))]);
    let foo = Value::Dict(vec![("a".into(), Value::Leaf(5)), ("bar".into(), bar)]);
    let root = Value::Dict(vec![("key".into(), Value::Leaf(3)), ("foo".into(), foo)]);

    let mut out = Vec::new();
    flatten(&root, "", &mut out);
    println!("{{");
    for (i, (k, v)) in out.iter().enumerate() {
        let comma = if i + 1 < out.len() { "," } else { "" };
        println!("    \"{}\": {}{}", k, v, comma);
    }
    println!("}}");
}
