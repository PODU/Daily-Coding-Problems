# Day 853: smallest distance (number of words between) between two words in a text.
# Single pass tracking last index of each word. distance = |i-j| - 1. O(n) time, O(1) space.

def min_distance(text, w1, w2):
    p1 = p2 = -1
    best = float("inf")
    for i, word in enumerate(text.split()):
        if word == w1:
            p1 = i
        if word == w2:
            p2 = i
        if p1 != -1 and p2 != -1:
            best = min(best, abs(p1 - p2) - 1)
    return best


if __name__ == "__main__":
    text = "dog cat hello cat dog dog hello cat world"
    print(min_distance(text, "hello", "world"))  # 1
