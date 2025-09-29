# Day 347: Lexicographically smallest string by moving one of first k letters to the end.
# k==1 -> best rotation; k>=2 -> any permutation reachable, so sorted. Time O(N^2)/O(N log N).
def smallest_string(s, k):
    if k == 1:
        return min(s[i:] + s[:i] for i in range(len(s)))
    return "".join(sorted(s))


def main():
    print(smallest_string("daily", 1))


if __name__ == "__main__":
    main()
