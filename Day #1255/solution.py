# Day 1255: Two-sum existence: one-pass hash set, check (k-num) seen before insert.
# Time O(n), Space O(n).
def has_pair(a, k):
    seen = set()
    for x in a:
        if k - x in seen:
            return True
        seen.add(x)
    return False

if __name__ == "__main__":
    v = [10, 15, 3, 7]
    print(str(has_pair(v, 17)).lower())
    print(str(has_pair(v, 50)).lower())
