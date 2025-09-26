// 24 Game (fixed order): recursive split of contiguous list, combine with +,-,*,/ (doubles).
// Time: O(1) for fixed 4 numbers. Space: O(1).
import java.util.*;

public class Solution {
    static List<Double> solve(double[] a, int lo, int hi) {
        List<Double> res = new ArrayList<>();
        if (hi - lo == 1) { res.add(a[lo]); return res; }
        for (int i = lo + 1; i < hi; ++i) {
            List<Double> L = solve(a, lo, i);
            List<Double> R = solve(a, i, hi);
            for (double l : L) for (double r : R) {
                res.add(l + r);
                res.add(l - r);
                res.add(l * r);
                if (Math.abs(r) > 1e-9) res.add(l / r);
            }
        }
        return res;
    }

    static boolean can24(double[] a) {
        for (double v : solve(a, 0, a.length))
            if (Math.abs(v - 24.0) < 1e-6) return true;
        return false;
    }

    public static void main(String[] args) {
        double[] nums = {5, 2, 7, 8};
        System.out.println(can24(nums) ? "True" : "False");
    }
}
