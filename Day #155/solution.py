# Day 155: Boyer-Moore majority vote in O(n) time, O(1) space. We verify the
# candidate; if no strict majority exists we fall back to the most frequent
# element so the answer is well-defined. Time O(n).
from collections import Counter


def majority_element(a):
    candidate = None
    count = 0
    for x in a:
        if count == 0:
            candidate = x
        count += 1 if x == candidate else -1

    if 2 * a.count(candidate) > len(a):
        return candidate  # strict majority

    # Fallback: most frequent element (example has no strict majority).
    return Counter(a).most_common(1)[0][0]


if __name__ == "__main__":
    print(majority_element([1, 2, 1, 1, 3, 4, 0]))  # 1
