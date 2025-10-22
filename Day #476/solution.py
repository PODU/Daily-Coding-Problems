# Day 476: Find duplicate in array of n+1 ints from 1..n using a seen set.
# Time O(n), Space O(n). (Floyd's cycle detection is an O(1)-space alternative.)
def find_duplicate(a):
    seen = set()
    for x in a:
        if x in seen:
            return x
        seen.add(x)
    return None


if __name__ == "__main__":
    print(find_duplicate([1, 3, 4, 2, 2]))
