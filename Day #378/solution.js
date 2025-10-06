// Custom JSON serializer for null/number/string/list/dict (recursive).
// Time: O(total size), Space: O(depth).
function encode(o) {
  if (o === null || o === undefined) return "null";
  const t = typeof o;
  if (t === "boolean") return o ? "true" : "false";
  if (t === "number") return String(o);
  if (t === "string") return '"' + o.replace(/\\/g, "\\\\").replace(/"/g, '\\"') + '"';
  if (Array.isArray(o)) return "[" + o.map(encode).join(", ") + "]";
  if (t === "object") {
    return "{" + Object.entries(o).map(([k, v]) => encode(String(k)) + ": " + encode(v)).join(", ") + "}";
  }
  throw new TypeError("Cannot encode " + t);
}

const data = [null, 123, ["a", "b"], { c: "d" }];
console.log("'" + encode(data) + "'");
