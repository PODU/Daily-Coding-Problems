# Day 1820: String rotation check: B is a rotation of A iff |A|==|B| and B is a substring of A+A.
# Time: O(n) (in operator). Space: O(n).


def is_rotation(a, b):
    return len(a) == len(b) and b in (a + a)


if __name__ == "__main__":
    print(str(is_rotation("abcde", "cdeab")).lower())  # true
    print(str(is_rotation("abc", "acb")).lower())      # false
