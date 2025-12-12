# Day 742: Flatten a nested dictionary, joining keys with '.' via DFS.
# Time: O(total keys), Space: O(depth) recursion.

def flatten(d, prefix=""):
    out = {}
    for k, v in d.items():
        key = f"{prefix}.{k}" if prefix else k
        if isinstance(v, dict):
            out.update(flatten(v, key))
        else:
            out[key] = v
    return out


if __name__ == "__main__":
    data = {
        "key": 3,
        "foo": {
            "a": 5,
            "bar": {
                "baz": 8
            }
        }
    }
    print(flatten(data))  # {'key': 3, 'foo.a': 5, 'foo.bar.baz': 8}
