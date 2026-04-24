# Day 1415: flatten a nested dictionary, namespacing keys with '.'.
# Approach: recursive DFS building prefixed keys. O(total keys) time/space.


def flatten(d, prefix="", out=None):
    if out is None:
        out = {}
    for k, v in d.items():
        key = f"{prefix}.{k}" if prefix else k
        if isinstance(v, dict):
            flatten(v, key, out)
        else:
            out[key] = v
    return out


if __name__ == "__main__":
    data = {"key": 3, "foo": {"a": 5, "bar": {"baz": 8}}}
    for k, v in flatten(data).items():
        print(f"{k}: {v}")
