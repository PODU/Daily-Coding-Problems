# Day 1516: Reverse words in a space-delimited string.
# Approach: split on spaces, reverse list, join. In-place variant reverses a char list.
# Time: O(n), Space: O(n) for Python strings (immutable).

def reverse_words(s: str) -> str:
    return " ".join(reversed(s.split(" ")))


if __name__ == "__main__":
    print('"' + reverse_words("hello world here") + '"')  # "here world hello"
