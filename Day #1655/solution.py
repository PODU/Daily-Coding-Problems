# Day 1655: Closure late-binding: buggy lambdas capture loop var by reference (all see final value);
# fix captures the value per-iteration via default arg. O(n) time/space.
print("Late binding (buggy):")
funcs = []
for i in [1, 2, 3]:
    funcs.append(lambda: i)
for f in funcs:
    print(f())
print("Fixed (capture value):")
funcs = []
for i in [1, 2, 3]:
    funcs.append(lambda i=i: i)
for f in funcs:
    print(f())
