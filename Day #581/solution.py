# Day 581: Rectangle intersection area via overlap of x and y ranges. Time O(1), Space O(1).
def intersection_area(tl1, dims1, tl2, dims2):
    left1, top1 = tl1
    right1, bottom1 = left1 + dims1[0], top1 - dims1[1]
    left2, top2 = tl2
    right2, bottom2 = left2 + dims2[0], top2 - dims2[1]
    overlap_w = min(right1, right2) - max(left1, left2)
    overlap_h = min(top1, top2) - max(bottom1, bottom2)
    return max(0, overlap_w) * max(0, overlap_h)


if __name__ == "__main__":
    print(intersection_area((1, 4), (3, 3), (0, 5), (4, 3)))
