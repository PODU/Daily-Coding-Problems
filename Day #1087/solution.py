# Day 1087: B is a rotation of A iff lengths match and B is a substring of A+A. Time O(n), Space O(n).
def is_rotation(a, b):
    return len(a) == len(b) and b in (a + a)


if __name__ == "__main__":
    print("true" if is_rotation("abcde", "cdeab") else "false")
    print("true" if is_rotation("abc", "acb") else "false")
