// Brute-force all O(k^2) pairs; rectangles overlap iff their projections strictly overlap on both axes.
// Sweep-line O(k log k) is possible but k^2 is clear. Time O(k^2), space O(k).
public class Solution {
    static class Rect {
        int x1, y1, x2, y2;
        Rect(int x, int y, int w, int h) { x1 = x; x2 = x + w; y2 = y; y1 = y - h; }
    }

    static boolean overlap(Rect a, Rect b) {
        return a.x1 < b.x2 && b.x1 < a.x2 && a.y1 < b.y2 && b.y1 < a.y2;
    }

    public static void main(String[] args) {
        Rect[] rects = {
            new Rect(1, 4, 3, 3),   // R1
            new Rect(-1, 3, 2, 1),  // R2
            new Rect(0, 5, 4, 3)    // R3
        };
        boolean any = false;
        outer:
        for (int i = 0; i < rects.length; i++)
            for (int j = i + 1; j < rects.length; j++)
                if (overlap(rects[i], rects[j])) { any = true; break outer; }
        System.out.println(any ? "true" : "false");
    }
}
