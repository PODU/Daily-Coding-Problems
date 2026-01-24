# Day 947: smallest sparse number (no two adjacent set bits) >= N.
# Repeatedly fix the lowest adjacent-1 pair by rounding up. Time O(log N), Space O(1).

def smallest_sparse(n):
    while n & (n >> 1):
        m = n & (n >> 1)            # bits where i and i+1 are both set
        p = (m & -m).bit_length() - 1  # lowest such position
        step = 1 << (p + 1)
        n = (n + step) & ~(step - 1)   # round up, clearing low bits
    return n


if __name__ == "__main__":
    print(smallest_sparse(21))  # 21 already sparse
    print(smallest_sparse(22))  # 32
