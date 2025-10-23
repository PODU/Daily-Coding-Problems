# Day 480: Word break reconstruction via memoized DP: for each suffix, try each prefix
# word and recurse. Time: O(n^2 * L) with memo, Space: O(n^2).


def reconstruct(words, s):
    dict_set = set(words)
    memo = {}

    def solve(start):
        if start == len(s):
            return []
        if start in memo:
            return memo[start]
        for end in range(start + 1, len(s) + 1):
            word = s[start:end]
            if word in dict_set:
                rest = solve(end)
                if rest is not None:
                    memo[start] = [word] + rest
                    return memo[start]
        memo[start] = None
        return None

    return solve(0)


def main():
    print(reconstruct(["quick", "brown", "the", "fox"], "thequickbrownfox"))
    print(reconstruct(["bed", "bath", "bedbath", "and", "beyond"], "bedbathandbeyond"))


if __name__ == "__main__":
    main()
