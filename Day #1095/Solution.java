// Detect if any pair of axis-aligned rectangles overlap (containment counts). Time O(n^2), Space O(n).
import java.util.*;

public class Solution {
    static class Rect { double minx, maxx, miny, maxy; Rect(double a, double b, double c, double d) { minx=a; maxx=b; miny=c; maxy=d; } }

    static Rect fromTopLeft(double x, double y, double w, double h) {
        return new Rect(x, x + w, y - h, y);
    }
    static boolean overlap(Rect a, Rect b) {
        return a.minx < b.maxx && b.minx < a.maxx && a.miny < b.maxy && b.miny < a.maxy;
    }
    static boolean anyOverlap(List<Rect> rs) {
        for (int i = 0; i < rs.size(); i++)
            for (int j = i + 1; j < rs.size(); j++)
                if (overlap(rs.get(i), rs.get(j))) return true;
        return false;
    }

    public static void main(String[] args) {
        List<Rect> rs = Arrays.asList(
            fromTopLeft(1, 4, 3, 3),
            fromTopLeft(-1, 3, 2, 1),
            fromTopLeft(0, 5, 4, 3));
        System.out.println(anyOverlap(rs) ? "true" : "false");
    }
}
