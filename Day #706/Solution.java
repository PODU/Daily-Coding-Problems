// Day 706: 24 Game (fixed order). Try every parenthesization over the fixed
// sequence, combining sub-results with +,-,*,/. Time ~O(1) for 4 numbers.
import java.util.*;

public class Solution {
    static List<Double> solve(double[] nums, int lo, int hi) {
        List<Double> res = new ArrayList<>();
        if (hi - lo == 1) { res.add(nums[lo]); return res; }
        for (int i = lo + 1; i < hi; i++) {
            List<Double> lv = solve(nums, lo, i);
            List<Double> rv = solve(nums, i, hi);
            for (double a : lv) for (double b : rv) {
                res.add(a + b);
                res.add(a - b);
                res.add(a * b);
                if (Math.abs(b) > 1e-9) res.add(a / b);
            }
        }
        return res;
    }

    static boolean game24(int[] digits) {
        double[] nums = new double[digits.length];
        for (int i = 0; i < digits.length; i++) nums[i] = digits[i];
        for (double v : solve(nums, 0, nums.length))
            if (Math.abs(v - 24.0) < 1e-6) return true;
        return false;
    }

    public static void main(String[] args) {
        int[] input = {5, 2, 7, 8};
        System.out.println(game24(input) ? "True" : "False");
    }
}
