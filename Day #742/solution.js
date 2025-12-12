// Flatten a nested dictionary, joining keys with '.' via DFS.
// Time: O(total keys), Space: O(depth) recursion.

function flatten(d, prefix = "") {
  const out = {};
  for (const [k, v] of Object.entries(d)) {
    const key = prefix ? `${prefix}.${k}` : k;
    if (v !== null && typeof v === "object" && !Array.isArray(v)) {
      Object.assign(out, flatten(v, key));
    } else {
      out[key] = v;
    }
  }
  return out;
}

const data = {
  key: 3,
  foo: {
    a: 5,
    bar: {
      baz: 8
    }
  }
};
console.log(flatten(data)); // { key: 3, 'foo.a': 5, 'foo.bar.baz': 8 }
