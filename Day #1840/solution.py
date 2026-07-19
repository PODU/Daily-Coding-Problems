# Day 1840: Flatten a nested dictionary, namespacing keys with '.'.
# Recursion over the dict; insertion order preserved. Time/Space O(total keys).


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


def main():
    nested = {"key": 3, "foo": {"a": 5, "bar": {"baz": 8}}}
    print(flatten(nested))


if __name__ == "__main__":
    main()
