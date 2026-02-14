# Day 1068: Dominoes final state via two-pass force summation. Time: O(n), Space: O(n).

def dominoes(s: str) -> str:
    n = len(s)
    forces = [0] * n
    # Left to right: R force propagates rightward
    f = 0
    for i in range(n):
        if   s[i] == 'R': f = n
        elif s[i] == 'L': f = 0
        else: f = max(0, f - 1)
        forces[i] += f
    # Right to left: L force propagates leftward (subtract)
    f = 0
    for i in range(n - 1, -1, -1):
        if   s[i] == 'L': f = n
        elif s[i] == 'R': f = 0
        else: f = max(0, f - 1)
        forces[i] -= f
    return ''.join('R' if v > 0 else 'L' if v < 0 else '.' for v in forces)

print(dominoes(".L.R....L"))
print(dominoes("..R...L.L"))
