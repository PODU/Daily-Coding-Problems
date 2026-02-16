// XOR all -> a^b; isolate via lowest set bit; partition & XOR each group to recover a,b; O(n) time O(1) space
public class Solution {
    static int[] findTwo(int[] nums) {
        int xorAll = 0;
        for (int n : nums) xorAll ^= n;
        int bit = xorAll & (-xorAll);          // lowest set bit differs between a and b
        int a = 0, b = 0;
        for (int n : nums) {
            if ((n & bit) != 0) a ^= n;
            else                b ^= n;
        }
        return new int[]{Math.min(a, b), Math.max(a, b)};
    }

    public static void main(String[] args) {
        int[] nums = {2, 4, 6, 8, 10, 2, 6, 10};
        int[] res = findTwo(nums);
        System.out.println(res[0] + " and " + res[1]);
    }
}
