# Day 378: Custom JSON serializer for null/number/string/list/dict (recursive).
# Time: O(total size), Space: O(depth).

def encode(o):
    if o is None:
        return "null"
    if isinstance(o, bool):
        return "true" if o else "false"
    if isinstance(o, (int, float)):
        return str(o)
    if isinstance(o, str):
        return '"' + o.replace("\\", "\\\\").replace('"', '\\"') + '"'
    if isinstance(o, list):
        return "[" + ", ".join(encode(x) for x in o) + "]"
    if isinstance(o, dict):
        return "{" + ", ".join(encode(str(k)) + ": " + encode(v) for k, v in o.items()) + "}"
    raise TypeError(f"Cannot encode {type(o)}")


if __name__ == "__main__":
    data = [None, 123, ["a", "b"], {"c": "d"}]
    print("'" + encode(data) + "'")
