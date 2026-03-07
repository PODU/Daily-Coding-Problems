// 24 game (fixed order): interval recursion combining left/right reachable
// values with + - * / using doubles + epsilon.
// Time: O(n^3 * S^2), Space: O(n^2 * S). Here n=4.
import java.util.*;

public class Solution {
    static List<Double> solve(int[] a, int l, int r) {
        List<Double> res = new ArrayList<>();
        if (l == r) { res.add((double) a[l]); return res; }
        for (int m = l; m < r; m++) {
            List<Double> L = solve(a, l, m);
            List<Double> R = solve(a, m + 1, r);
            for (double x : L) for (double y : R) {
                res.add(x + y);
                res.add(x - y);
                res.add(x * y);
                if (Math.abs(y) > 1e-9) res.add(x / y);
            }
        }
        return res;
    }

    static boolean can24(int[] a) {
        for (double v : solve(a, 0, a.length - 1))
            if (Math.abs(v - 24.0) < 1e-6) return true;
        return false;
    }

    public static void main(String[] args) {
        int[] a = {5, 2, 7, 8};
        System.out.println(can24(a) ? "True" : "False");
    }
}
