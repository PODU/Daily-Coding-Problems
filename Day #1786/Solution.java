// Recursively combine adjacent pairs (preserves order, covers all parenthesizations)
// applying +,-,*,/ until one value remains; check |v-24|<1e-6.
// Time O(exponential in n), Space O(n) recursion. Here n=4.
import java.util.*;

public class Solution {
    static boolean solve(List<Double> nums) {
        if (nums.size() == 1) return Math.abs(nums.get(0) - 24.0) < 1e-6;
        for (int i = 0; i + 1 < nums.size(); i++) {
            double a = nums.get(i), b = nums.get(i + 1);
            List<Double> results = new ArrayList<>(Arrays.asList(a + b, a - b, a * b));
            if (Math.abs(b) > 1e-9) results.add(a / b);
            for (double r : results) {
                List<Double> next = new ArrayList<>();
                for (int j = 0; j < nums.size(); j++) {
                    if (j == i) next.add(r);
                    else if (j == i + 1) continue;
                    else next.add(nums.get(j));
                }
                if (solve(next)) return true;
            }
        }
        return false;
    }

    static boolean canGet24(int[] nums) {
        List<Double> d = new ArrayList<>();
        for (int n : nums) d.add((double) n);
        return solve(d);
    }

    public static void main(String[] args) {
        System.out.println(canGet24(new int[]{5, 2, 7, 8}) ? "True" : "False");
    }
}
