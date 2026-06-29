// XOR all -> a^b; partition by a set bit, XOR each group to recover the two singletons. O(n) time, O(1) space.
public class Solution {
    public static void main(String[] args) {
        int[] nums = {2, 4, 6, 8, 10, 2, 6, 10};
        int xorAll = 0;
        for (int x : nums) xorAll ^= x;
        int bit = xorAll & (-xorAll); // lowest set bit
        int a = 0, b = 0;
        for (int x : nums) {
            if ((x & bit) != 0) a ^= x; else b ^= x;
        }
        if (a > b) { int t = a; a = b; b = t; }
        System.out.println(a + " and " + b);
    }
}
