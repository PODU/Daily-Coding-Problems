// Flatten nested dict, namespacing keys with '.'. Recurse; on object value prepend
// parentKey + '.'. JS object preserves string-key insertion order.
// Time O(total entries), Space O(depth).
function flatten(d, prefix, out) {
    for (const k of Object.keys(d)) {
        const key = prefix + k;
        const v = d[k];
        if (v !== null && typeof v === "object" && !Array.isArray(v)) {
            flatten(v, key + ".", out);
        } else {
            out[key] = v;
        }
    }
    return out;
}

function main() {
    const data = { key: 3, foo: { a: 5, bar: { baz: 8 } } };
    const out = flatten(data, "", {});
    const parts = Object.keys(out).map(k => `"${k}": ${out[k]}`);
    console.log("{" + parts.join(", ") + "}");
}

main();
