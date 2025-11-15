# Day 600: Closest pair of points on a plane.
# Approach: divide & conquer with a y-sorted strip check. Time O(n log n), Space O(n).
import math


def dist2(a, b):
    return (a[0] - b[0]) ** 2 + (a[1] - b[1]) ** 2


def closest(px):
    def rec(lo, hi):
        n = hi - lo
        if n <= 3:
            best = float('inf')
            bp = None
            for i in range(lo, hi):
                for j in range(i + 1, hi):
                    d = dist2(px[i], px[j])
                    if d < best:
                        best, bp = d, (px[i], px[j])
            return best, bp
        mid = (lo + hi) // 2
        midx = px[mid][0]
        bl, pl = rec(lo, mid)
        br, pr = rec(mid, hi)
        best, bp = (bl, pl) if bl <= br else (br, pr)
        dd = math.sqrt(best)
        strip = [px[i] for i in range(lo, hi) if abs(px[i][0] - midx) < dd]
        strip.sort(key=lambda p: p[1])
        for i in range(len(strip)):
            j = i + 1
            while j < len(strip) and (strip[j][1] - strip[i][1]) < dd:
                d = dist2(strip[i], strip[j])
                if d < best:
                    best, bp = d, (strip[i], strip[j])
                    dd = math.sqrt(best)
                j += 1
        return best, bp

    return rec(0, len(px))


def main():
    pts = [(1, 1), (-1, -1), (3, 4), (6, 1), (-1, -6), (-4, -3)]
    pts.sort()
    _, (a, b) = closest(pts)
    if a > b:
        a, b = b, a
    print(f"[{a}, {b}]")


if __name__ == '__main__':
    main()
