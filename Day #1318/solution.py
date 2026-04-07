# Day 1318: Move one of first k chars to the end, unlimited times.
# k==1: only rotations reachable -> smallest rotation. k>=2: any order -> sort.
# Time O(n^2) for k==1 (rotation scan), O(n log n) for k>=2.

def smallest_string(s, k):
    if k >= 2:
        return "".join(sorted(s))
    return min(s[i:] + s[:i] for i in range(len(s)))


if __name__ == "__main__":
    print(smallest_string("daily", 1))  # ailyd
