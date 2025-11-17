# Day 617: Roman numeral -> decimal. Single left-to-right pass; subtract when a smaller
# value precedes a larger one. Time O(n), Space O(1).

def roman_to_int(s):
    val = {'M': 1000, 'D': 500, 'C': 100, 'L': 50, 'X': 10, 'V': 5, 'I': 1}
    total = 0
    for i, ch in enumerate(s):
        v = val[ch]
        if i + 1 < len(s) and val[s[i + 1]] > v:
            total -= v
        else:
            total += v
    return total


if __name__ == "__main__":
    print(roman_to_int("XIV"))  # 14
