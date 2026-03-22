# Day 1248: Concatenation substring indices via sliding window over wordLen offsets with hashmap counts. O(n*wordLen) time, O(m) space.
from collections import Counter


def find_substring(s, words):
    res = []
    if not words or not s:
        return res
    word_len = len(words[0])
    num_words = len(words)
    window_len = word_len * num_words
    if len(s) < window_len:
        return res

    need = Counter(words)

    for offset in range(word_len):
        window = Counter()
        count = 0
        left = offset
        for right in range(offset, len(s) - word_len + 1, word_len):
            word = s[right:right + word_len]
            if word in need:
                window[word] += 1
                count += 1
                while window[word] > need[word]:
                    lw = s[left:left + word_len]
                    window[lw] -= 1
                    count -= 1
                    left += word_len
                if count == num_words:
                    res.append(left)
                    lw = s[left:left + word_len]
                    window[lw] -= 1
                    count -= 1
                    left += word_len
            else:
                window.clear()
                count = 0
                left = right + word_len
    res.sort()
    return res


def main():
    s = "dogcatcatcodecatdog"
    words = ["cat", "dog"]
    res = find_substring(s, words)
    print("[" + ", ".join(str(x) for x in res) + "]")


if __name__ == "__main__":
    main()
