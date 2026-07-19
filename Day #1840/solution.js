// Day 1840: Flatten a nested dictionary, namespacing keys with '.'.
// Recursion over the object; insertion order preserved. Time/Space O(total keys).

function flatten(obj, prefix = "", out = {}) {
  for (const [k, v] of Object.entries(obj)) {
    const key = prefix ? `${prefix}.${k}` : k;
    if (v !== null && typeof v === "object" && !Array.isArray(v)) flatten(v, key, out);
    else out[key] = v;
  }
  return out;
}

function main() {
  const nested = { key: 3, foo: { a: 5, bar: { baz: 8 } } };
  console.log(flatten(nested));
}

main();
