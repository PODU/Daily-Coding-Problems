# Day 1243: A rotation of B iff same length and B is a substring of A+A.
# Time O(n) (with fast substring search), Space O(n).


def is_rotation(a, b):
    return len(a) == len(b) and b in (a + a)


if __name__ == "__main__":
    print(str(is_rotation("abcde", "cdeab")).lower())
    print(str(is_rotation("abc", "acb")).lower())
