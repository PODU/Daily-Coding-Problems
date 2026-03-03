# Day 1146: Dominoes - two-pass force accumulation.
# L->R pass adds rightward force, R->L pass adds leftward; sign decides. O(n) time, O(n) space.
def push_dominoes(s):
    n = len(s)
    forces = [0] * n
    force = 0
    for i in range(n):
        if s[i] == "R":
            force = n
        elif s[i] == "L":
            force = 0
        else:
            force = max(force - 1, 0)
        forces[i] += force
    force = 0
    for i in range(n - 1, -1, -1):
        if s[i] == "L":
            force = n
        elif s[i] == "R":
            force = 0
        else:
            force = max(force - 1, 0)
        forces[i] -= force
    return "".join("R" if f > 0 else "L" if f < 0 else "." for f in forces)


if __name__ == "__main__":
    print(push_dominoes(".L.R....L"))  # LL.RRRLLL
    print(push_dominoes("..R...L.L"))  # ..RR.LLLL
