# Day 1570: Look-and-say: build each term by run-length encoding the previous. Time O(total digits), space O(len).
def look_and_say(n):
    cur = "1"
    for _ in range(n - 1):
        parts = []
        i, length = 0, len(cur)
        while i < length:
            j = i
            while j < length and cur[j] == cur[i]:
                j += 1
            parts.append(str(j - i))
            parts.append(cur[i])
            i = j
        cur = "".join(parts)
    return cur


if __name__ == "__main__":
    print(look_and_say(4))  # 1, 11, 21, 1211
