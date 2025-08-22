# Day 153: Min words separating two words. Single pass tracking last seen index
# of each word; answer is min(|i-j|-1). Time O(n), Space O(1).


def min_distance(words, a, b):
    last_a = last_b = -1
    best = float("inf")
    for i, w in enumerate(words):
        if w == a:
            last_a = i
            if last_b != -1:
                best = min(best, abs(last_a - last_b) - 1)
        if w == b:
            last_b = i
            if last_a != -1:
                best = min(best, abs(last_a - last_b) - 1)
    return best


if __name__ == "__main__":
    text = "dog cat hello cat dog dog hello cat world"
    print(min_distance(text.split(), "hello", "world"))
