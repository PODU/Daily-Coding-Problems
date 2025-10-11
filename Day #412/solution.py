# Day 412: Nth term of the look-and-say sequence via run-length encoding.
# Build each term from the previous by counting consecutive runs. O(N * L) time where L = term length, O(L) space.
def look_and_say(n):
    cur = "1"
    for _ in range(n - 1):
        parts = []
        i, m = 0, len(cur)
        while i < m:
            j = i
            while j < m and cur[j] == cur[i]:
                j += 1
            parts.append(str(j - i))
            parts.append(cur[i])
            i = j
        cur = "".join(parts)
    return cur


if __name__ == "__main__":
    n = 4
    print(look_and_say(n))
