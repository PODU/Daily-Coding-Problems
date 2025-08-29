// Day 185: Area of intersection of two axis-aligned rectangles (top-left + width/height, y up).
// Overlap = max(0, dx) * max(0, dy). Time O(1), Space O(1).
public class Solution {
    static class Rect { int left, top, w, h; Rect(int l, int t, int w, int h){left=l;top=t;this.w=w;this.h=h;} }

    static long intersectionArea(Rect a, Rect b) {
        int ox = Math.min(a.left + a.w, b.left + b.w) - Math.max(a.left, b.left);
        int oy = Math.min(a.top, b.top) - Math.max(a.top - a.h, b.top - b.h);
        if (ox <= 0 || oy <= 0) return 0;
        return (long) ox * oy;
    }

    public static void main(String[] args) {
        Rect a = new Rect(1, 4, 3, 3);
        Rect b = new Rect(0, 5, 4, 3);
        System.out.println(intersectionArea(a, b));
    }
}
