# Day 1594: Approach: Pairwise O(n^2) overlap check on axis-aligned rectangles (strict, positive-area).
# Time O(n^2), Space O(1).

def make(x, y, w, h):
    # top_left (x,y), dims (w,h): x in [x,x+w], y in [y-h,y]
    return (x, y - h, x + w, y)  # (x1, y1, x2, y2)

def overlap(a, b):
    return a[0] < b[2] and b[0] < a[2] and a[1] < b[3] and b[1] < a[3]

def any_overlap(rs):
    n = len(rs)
    for i in range(n):
        for j in range(i + 1, n):
            if overlap(rs[i], rs[j]):
                return True
    return False

if __name__ == "__main__":
    rs = [make(1, 4, 3, 3), make(-1, 3, 2, 1), make(0, 5, 4, 3)]
    print("true" if any_overlap(rs) else "false")
