# Day 562: Product of array except self without division.
# Prefix * suffix products in two passes. Time O(N), Space O(1) extra (besides output).
def product_except_self(a):
    n = len(a)
    res = [1] * n
    prefix = 1
    for i in range(n):
        res[i] = prefix
        prefix *= a[i]
    suffix = 1
    for i in range(n - 1, -1, -1):
        res[i] *= suffix
        suffix *= a[i]
    return res


def fmt(v):
    return "[" + ", ".join(str(x) for x in v) + "]"


if __name__ == "__main__":
    print(fmt(product_except_self([1, 2, 3, 4, 5])))
    print(fmt(product_except_self([3, 2, 1])))
