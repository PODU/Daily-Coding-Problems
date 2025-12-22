# Day 779: Egg drop - minimum worst-case trials for N eggs, k floors.
# Find smallest m with sum_{i=1..N} C(m,i) >= k. O(N * answer) time, O(1) space.


def egg_drop(eggs, floors):
    if floors == 0:
        return 0
    m = 0
    while True:
        m += 1
        reach, c = 0, 1
        for i in range(1, eggs + 1):
            c = c * (m - i + 1) // i
            reach += c
            if reach >= floors:
                break
        if reach >= floors:
            return m


if __name__ == "__main__":
    print(egg_drop(1, 5))    # 5
    print(egg_drop(2, 100))  # 14
