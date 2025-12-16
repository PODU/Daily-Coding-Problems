# Day 747: B is a rotation of A iff len(A)==len(B) and B is a substring of A+A.
# Time: O(n) (Python 'in' uses Two-Way / Crochemore-Perrin), Space: O(n).

def is_rotation(a, b):
    return len(a) == len(b) and b in (a + a)


if __name__ == "__main__":
    print(is_rotation("abcde", "cdeab"))  # True
    print(is_rotation("abc", "acb"))      # False
