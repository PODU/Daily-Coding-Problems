# Day 1321: Roman numeral -> decimal.
# Approach: single left-to-right pass; subtract if a smaller value precedes a larger one. O(n) time, O(1) space.

def roman_to_int(s):
    v = {'M': 1000, 'D': 500, 'C': 100, 'L': 50, 'X': 10, 'V': 5, 'I': 1}
    total = 0
    for i, ch in enumerate(s):
        cur = v[ch]
        if i + 1 < len(s) and cur < v[s[i + 1]]:
            total -= cur
        else:
            total += cur
    return total


if __name__ == "__main__":
    print(roman_to_int("XIV"))  # 14
