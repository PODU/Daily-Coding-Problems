// 24 game, fixed order: recursively split the sequence at each position, combine left/right results with +,-,*,/ (eps for div). O(4^n) over splits; O(n) depth.
import java.util.*;

public class Solution {
    static final double EPS = 1e-6;

    static List<Double> solve(double[] nums, int lo, int hi) {
        if (hi - lo == 1) return Arrays.asList(nums[lo]);
        List<Double> res = new ArrayList<>();
        for (int mid = lo + 1; mid < hi; mid++) {
            List<Double> L = solve(nums, lo, mid);
            List<Double> R = solve(nums, mid, hi);
            for (double a : L) for (double b : R) {
                res.add(a + b);
                res.add(a - b);
                res.add(a * b);
                if (Math.abs(b) > EPS) res.add(a / b);
            }
        }
        return res;
    }

    static boolean canReach24(int[] in) {
        double[] nums = new double[in.length];
        for (int i = 0; i < in.length; i++) nums[i] = in[i];
        for (double v : solve(nums, 0, nums.length))
            if (Math.abs(v - 24.0) < EPS) return true;
        return false;
    }

    public static void main(String[] args) {
        int[] input = {5, 2, 7, 8};
        System.out.println(canReach24(input) ? "True" : "False");
    }
}
