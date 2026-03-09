# Day 1185: Smallest distance (words in between) between two words in a text.
# Single pass tracking last index of each target word; min |i-j| - 1.
# Time O(N), Space O(1).


def shortest_distance(text, w1, w2):
    p1 = p2 = -1
    best = float("inf")
    for i, token in enumerate(text.split()):
        if token == w1:
            p1 = i
        if token == w2:
            p2 = i
        if p1 >= 0 and p2 >= 0:
            best = min(best, abs(p1 - p2))
    return -1 if best == float("inf") else best - 1


if __name__ == "__main__":
    text = "dog cat hello cat dog dog hello cat world"
    print(shortest_distance(text, "hello", "world"))  # 1
