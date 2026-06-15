# Day 1668: Iterate counting integers; pick those whose digit sum == 10. O(answer) time, O(1) space.
def nth_perfect(n):
    x, c = 0, 0
    while c < n:
        x += 1
        if sum(int(d) for d in str(x)) == 10:
            c += 1
    return x

print(nth_perfect(1))
print(nth_perfect(2))
