// Single number among triples: sum each bit position mod 3 to rebuild the unique value. O(N) time, O(1) space.
public class Solution {
    static int singleNumber(int[] nums) {
        int result = 0;
        for (int b = 0; b < 32; b++) {
            int cnt = 0;
            for (int x : nums) cnt += (x >> b) & 1;
            if (cnt % 3 != 0) result |= (1 << b);
        }
        return result;
    }

    public static void main(String[] args) {
        System.out.println(singleNumber(new int[]{6, 1, 3, 3, 3, 6, 6}));
        System.out.println(singleNumber(new int[]{13, 19, 13, 13}));
    }
}
