// XOR all -> a^b; isolate a differing bit; partition into two groups and XOR each.
// Time O(n); Space O(1).
public class Solution {
    static int[] twoSingles(int[] nums) {
        int x = 0;
        for (int v : nums) x ^= v;
        int bit = x & (-x); // lowest set bit where the two singles differ
        int a = 0, b = 0;
        for (int v : nums) {
            if ((v & bit) != 0) a ^= v;
            else b ^= v;
        }
        if (a > b) { int t = a; a = b; b = t; }
        return new int[]{a, b};
    }

    public static void main(String[] args) {
        int[] nums = {2, 4, 6, 8, 10, 2, 6, 10};
        int[] r = twoSingles(nums);
        System.out.println(r[0] + " and " + r[1]); // 4 and 8
    }
}
