// Day 1331: Does any pair of rectangles overlap (full containment counts; edge-touching does not)?
// Convert top_left+dimensions to [xmin,xmax,ymin,ymax]; pairwise strict-interval overlap test. O(n^2).
import java.util.*;

public class Solution {
    static long[] make(long tlx, long tly, long w, long h) {
        return new long[]{tlx, tlx + w, tly - h, tly}; // xmin, xmax, ymin, ymax
    }

    static boolean overlap(long[] a, long[] b) {
        return a[0] < b[1] && b[0] < a[1] && a[2] < b[3] && b[2] < a[3];
    }

    public static void main(String[] args) {
        long[][] rs = {
            make(1, 4, 3, 3),
            make(-1, 3, 2, 1),
            make(0, 5, 4, 3),
        };
        boolean any = false;
        for (int i = 0; i < rs.length && !any; i++)
            for (int j = i + 1; j < rs.length; j++)
                if (overlap(rs[i], rs[j])) { any = true; break; }
        System.out.println(any); // true
    }
}
