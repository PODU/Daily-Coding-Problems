# Day 322: Min jumps on number line: smallest k with S=k(k+1)/2 >= |N| and (S-|N|) even.
# Time: O(sqrt N), Space: O(1).

def min_jumps(N):
    target = abs(N)
    k = 0
    S = 0
    while S < target or (S - target) % 2 != 0:
        k += 1
        S += k
    return k


if __name__ == "__main__":
    N = 10
    print(f"Minimum jumps to reach {N}: {min_jumps(N)}")
