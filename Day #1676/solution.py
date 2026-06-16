# Day 1676: PrefixMapSum via trie storing cumulative values + delta on overwrite.
# insert/sum both O(key length). Space O(total chars).


class PrefixMapSum:
    def __init__(self):
        self.total = {}   # prefix -> cumulative sum
        self.vals = {}    # key -> value

    def insert(self, key, value):
        delta = value - self.vals.get(key, 0)
        self.vals[key] = value
        prefix = ""
        for ch in key:
            prefix += ch
            self.total[prefix] = self.total.get(prefix, 0) + delta

    def sum(self, prefix):
        return self.total.get(prefix, 0)


if __name__ == "__main__":
    m = PrefixMapSum()
    m.insert("columnar", 3)
    print(m.sum("col"))  # 3
    m.insert("column", 2)
    print(m.sum("col"))  # 5
