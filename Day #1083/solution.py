# Day 1083: Roman to decimal: add each value, subtract when a smaller numeral precedes a larger. Time O(n), Space O(1).
def roman_to_int(s):
    v = {'M': 1000, 'D': 500, 'C': 100, 'L': 50, 'X': 10, 'V': 5, 'I': 1}
    total = 0
    for i, ch in enumerate(s):
        if i + 1 < len(s) and v[ch] < v[s[i + 1]]:
            total -= v[ch]
        else:
            total += v[ch]
    return total


if __name__ == "__main__":
    print(roman_to_int("XIV"))
