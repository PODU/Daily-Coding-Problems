// Flatten nested dictionary, namespacing keys with '.'. Recursive DFS preserving insertion order.
// Time O(total keys), Space O(total keys).
function flatten(obj, prefix = "", out = {}) {
  for (const [k, v] of Object.entries(obj)) {
    const key = prefix ? `${prefix}.${k}` : k;
    if (v !== null && typeof v === "object" && !Array.isArray(v)) {
      flatten(v, key, out);
    } else {
      out[key] = v;
    }
  }
  return out;
}

const data = { key: 3, foo: { a: 5, bar: { baz: 8 } } };
for (const [k, v] of Object.entries(flatten(data))) {
  console.log(`${k}: ${v}`);
}
