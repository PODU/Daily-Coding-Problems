// Custom JSON serializer for null/number/string/list/dict (recursive).
// Time: O(total size), Space: O(depth).

enum Json {
    Null,
    Int(i64),
    Str(String),
    List(Vec<Json>),
    Dict(Vec<(String, Json)>), // ordered
}

fn esc(s: &str) -> String {
    let mut o = String::from("\"");
    for c in s.chars() {
        if c == '"' || c == '\\' {
            o.push('\\');
        }
        o.push(c);
    }
    o.push('"');
    o
}

fn encode(v: &Json) -> String {
    match v {
        Json::Null => "null".to_string(),
        Json::Int(n) => n.to_string(),
        Json::Str(s) => esc(s),
        Json::List(items) => {
            let parts: Vec<String> = items.iter().map(encode).collect();
            format!("[{}]", parts.join(", "))
        }
        Json::Dict(entries) => {
            let parts: Vec<String> = entries
                .iter()
                .map(|(k, val)| format!("{}: {}", esc(k), encode(val)))
                .collect();
            format!("{{{}}}", parts.join(", "))
        }
    }
}

fn main() {
    let data = Json::List(vec![
        Json::Null,
        Json::Int(123),
        Json::List(vec![Json::Str("a".into()), Json::Str("b".into())]),
        Json::Dict(vec![("c".into(), Json::Str("d".into()))]),
    ]);
    println!("'{}'", encode(&data));
}
