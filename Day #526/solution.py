# Day 526: k==1: smallest rotation (try all n rotations). k>=2: sorted string (any pair
# of front letters can be reordered). Time O(n^2) for k==1, O(n log n) k>=2.


def smallest(s, k):
    if k >= 2:
        return "".join(sorted(s))
    n = len(s)
    return min(s[i:] + s[:i] for i in range(n))


if __name__ == "__main__":
    print(smallest("daily", 1))
