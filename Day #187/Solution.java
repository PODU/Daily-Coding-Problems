// Day 187: Do any two rectangles overlap (containment counts; edge-touching does not).
// Pairwise interior-overlap test. Time O(n^2), Space O(1). (Sweep line gives O(n log n).)
public class Solution {
    static class Rect { int left, top, w, h; Rect(int l,int t,int w,int h){left=l;top=t;this.w=w;this.h=h;} }

    static boolean overlap(Rect a, Rect b) {
        int ox = Math.min(a.left + a.w, b.left + b.w) - Math.max(a.left, b.left);
        int oy = Math.min(a.top, b.top) - Math.max(a.top - a.h, b.top - b.h);
        return ox > 0 && oy > 0;
    }

    static boolean anyOverlap(Rect[] rs) {
        for (int i = 0; i < rs.length; i++)
            for (int j = i + 1; j < rs.length; j++)
                if (overlap(rs[i], rs[j])) return true;
        return false;
    }

    public static void main(String[] args) {
        Rect[] rs = { new Rect(1,4,3,3), new Rect(-1,3,2,1), new Rect(0,5,4,3) };
        System.out.println(anyOverlap(rs));
    }
}
