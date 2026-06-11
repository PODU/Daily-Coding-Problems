# Day 1642: Min word distance: single pass tracking last-seen index of each word; on each
# hit, update min with |i-j|-1 (words strictly between). Time O(n), Space O(1).

def min_word_distance(text, w1, w2):
    last1, last2, best = -1, -1, float("inf")
    for i, word in enumerate(text):
        if word == w1:
            last1 = i
            if last2 != -1:
                best = min(best, abs(last1 - last2) - 1)
        if word == w2:
            last2 = i
            if last1 != -1:
                best = min(best, abs(last1 - last2) - 1)
    return best


if __name__ == "__main__":
    text = "dog cat hello cat dog dog hello cat world".split()
    print(min_word_distance(text, "hello", "world"))
