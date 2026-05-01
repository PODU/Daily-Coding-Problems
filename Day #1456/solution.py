# Day 1456: First recurring character in a string.
# Approach: scan left-to-right, track seen chars in a set; first char already
# seen is the answer. Time O(n), Space O(1) (fixed alphabet).
from typing import Optional


def first_recurring(s: str) -> Optional[str]:
    seen = set()
    for c in s:
        if c in seen:
            return c
        seen.add(c)
    return None


if __name__ == "__main__":
    for s in ["acbbac", "abcdef"]:
        r = first_recurring(s)
        print(f'"{r}"' if r is not None else "null")
