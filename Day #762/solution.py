# Day 762: Smallest distance (words in between) between two target words.
# Single pass tracking last seen index of each word. Time: O(n), Space: O(1).


def smallest_distance(words, a, b):
    last_a = last_b = -1
    best_gap = float("inf")
    for i, w in enumerate(words):
        if w == a:
            last_a = i
            if last_b != -1:
                best_gap = min(best_gap, last_a - last_b)
        if w == b:
            last_b = i
            if last_a != -1:
                best_gap = min(best_gap, last_b - last_a)
    if best_gap == float("inf"):
        return -1
    return best_gap - 1   # number of words strictly between


if __name__ == "__main__":
    text = "dog cat hello cat dog dog hello cat world"
    print(smallest_distance(text.split(), "hello", "world"))  # 1
