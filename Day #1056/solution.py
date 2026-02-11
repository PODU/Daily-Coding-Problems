# Day 1056: Look-and-say sequence: start "1"; each term describes digit runs of previous.
# Iteratively build N terms by run-length encoding. Time O(N * L), Space O(L).


def look_and_say(n):
    cur = "1"
    for _ in range(n - 1):
        parts = []
        j = 0
        while j < len(cur):
            count = 1
            while j + count < len(cur) and cur[j + count] == cur[j]:
                count += 1
            parts.append(str(count))
            parts.append(cur[j])
            j += count
        cur = "".join(parts)
    return cur


def main():
    N = 5  # terms: 1, 11, 21, 1211, 111221
    print(look_and_say(N))


if __name__ == "__main__":
    main()
