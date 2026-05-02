# Day 1461: Egg drop: DP on trials. f(t,e)=f(t-1,e-1)+f(t-1,e)+1 = max floors with t trials, e eggs.
# Smallest t with f(t,N)>=k. Time O(N*answer), Space O(N).


def egg_drop(N, k):
    if k == 0:
        return 0
    f = [0] * (N + 1)  # floors solvable with current t trials, e eggs
    t = 0
    while f[N] < k:
        t += 1
        for e in range(N, 0, -1):
            f[e] = f[e] + f[e - 1] + 1
    return t


def main():
    print(egg_drop(1, 5))


if __name__ == '__main__':
    main()
