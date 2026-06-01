// Approach: Pairwise O(n^2) overlap check on axis-aligned rectangles (strict, positive-area).
// Time O(n^2), Space O(1).
import java.util.*;

public class Solution {
    static class Rect { double x1, y1, x2, y2; Rect(double a,double b,double c,double d){x1=a;y1=b;x2=c;y2=d;} }

    static boolean overlap(Rect a, Rect b) {
        return a.x1 < b.x2 && b.x1 < a.x2 && a.y1 < b.y2 && b.y1 < a.y2;
    }

    static boolean anyOverlap(List<Rect> rs) {
        for (int i = 0; i < rs.size(); i++)
            for (int j = i + 1; j < rs.size(); j++)
                if (overlap(rs.get(i), rs.get(j))) return true;
        return false;
    }

    // top_left (x,y), dims (w,h): x in [x,x+w], y in [y-h,y]
    static Rect make(double x, double y, double w, double h) { return new Rect(x, y - h, x + w, y); }

    public static void main(String[] args) {
        List<Rect> rs = Arrays.asList(
            make(1, 4, 3, 3),
            make(-1, 3, 2, 1),
            make(0, 5, 4, 3)
        );
        System.out.println(anyOverlap(rs) ? "true" : "false");
    }
}
