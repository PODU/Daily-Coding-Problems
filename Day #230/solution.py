# Day 230: Egg drop: find min trials t such that maxFloors(t, eggs) >= k, where
# f(t,e) = f(t-1,e-1) + f(t-1,e) + 1 (floors distinguishable). Time: O(eggs * answer), Space: O(eggs).


def egg_drop(eggs, k):
    f = [0] * (eggs + 1)
    t = 0
    while f[eggs] < k:
        t += 1
        for e in range(eggs, 0, -1):
            f[e] = f[e] + f[e - 1] + 1
    return t


if __name__ == "__main__":
    print(egg_drop(1, 5))  # 5
