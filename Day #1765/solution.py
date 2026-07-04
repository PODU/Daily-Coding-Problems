# Day 1765: Roman numeral -> decimal: scan left to right; subtract when a symbol's value is
# less than the next symbol's, otherwise add. Time: O(n), Space: O(1).

def roman_to_int(s):
    v = {'M': 1000, 'D': 500, 'C': 100, 'L': 50, 'X': 10, 'V': 5, 'I': 1}
    total = 0
    for i in range(len(s)):
        if i + 1 < len(s) and v[s[i]] < v[s[i + 1]]:
            total -= v[s[i]]
        else:
            total += v[s[i]]
    return total


if __name__ == "__main__":
    print(roman_to_int("XIV"))
