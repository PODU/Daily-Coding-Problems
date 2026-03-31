// Day 1281: Area of intersection of two axis-aligned rectangles.
// Overlap on each axis = min(rights)-max(lefts), clamped at 0. Time O(1), Space O(1).
public class Solution {
    // Each rectangle: top-left (x, y) with width w and height h (y grows upward).
    static long intersectArea(int x1, int y1, int w1, int h1, int x2, int y2, int w2, int h2) {
        int ow = Math.min(x1 + w1, x2 + w2) - Math.max(x1, x2);
        int oh = Math.min(y1, y2) - Math.max(y1 - h1, y2 - h2);
        if (ow <= 0 || oh <= 0) return 0;
        return (long) ow * oh;
    }

    public static void main(String[] args) {
        System.out.println(intersectArea(1, 4, 3, 3, 0, 5, 4, 3)); // 6
    }
}
