# Day 108: B is a rotation of A iff |A|==|B| and B is a substring of A+A. O(n).
def is_rotation(a, b):
    return len(a) == len(b) and b in (a + a)


if __name__ == "__main__":
    print(str(is_rotation("abcde", "cdeab")).lower())
    print(str(is_rotation("abc", "acb")).lower())
