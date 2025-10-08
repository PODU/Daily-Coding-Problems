# Day 390: Presence bitset: mark each present value, then report unmarked ones.
# Time: O(N), Space: O(N) bits.  (N = 1,000,000)

def find_missing(present, N):
    seen = bytearray(N + 1)
    for x in present:
        seen[x] = 1
    return [i for i in range(1, N + 1) if not seen[i]]


if __name__ == "__main__":
    N = 1000000
    present = [i for i in range(1, N + 1) if i % 1000 != 0]
    missing = find_missing(present, N)
    print("Missing count:", len(missing))
    print("First 10 missing:", " ".join(map(str, missing[:10])))
    print("Time complexity: O(N), Space complexity: O(N) bits (N = 1,000,000)")
