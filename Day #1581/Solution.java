// Day 1581: Area of intersection of two axis-aligned rectangles.
// top_left=(x,y), dims=(w,h); x-range [x,x+w], y-range [y-h,y]. Overlap = clamped widths.
// Time: O(1); Space: O(1).
public class Solution {
    static long intersectionArea(int x1, int y1, int w1, int h1, int x2, int y2, int w2, int h2) {
        long ow = Math.min(x1 + w1, x2 + w2) - Math.max(x1, x2);
        long oh = Math.min(y1, y2) - Math.max(y1 - h1, y2 - h2);
        if (ow <= 0 || oh <= 0) return 0;
        return ow * oh;
    }

    public static void main(String[] args) {
        System.out.println(intersectionArea(1, 4, 3, 3, 0, 5, 4, 3)); // 6
    }
}
