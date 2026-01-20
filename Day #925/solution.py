# Day 925: Flatten nested dict, namespacing keys with '.'. Recurse; on dict value prepend
# parentKey + '.'. Insertion order preserved. Time O(total entries), Space O(depth).

def flatten(d, prefix=""):
    result = {}
    for k, v in d.items():
        key = prefix + k
        if isinstance(v, dict):
            result.update(flatten(v, key + "."))
        else:
            result[key] = v
    return result


def main():
    data = {"key": 3, "foo": {"a": 5, "bar": {"baz": 8}}}
    print(flatten(data))


if __name__ == "__main__":
    main()
