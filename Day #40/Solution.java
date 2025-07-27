// Single number where others appear 3x: ones/twos bit-counting state machine.
// O(N) time, O(1) space. Works for negatives via two's-complement int.
public class Solution {
    static int singleNumber(int[] nums) {
        int ones = 0, twos = 0;
        for (int x : nums) {
            ones = (ones ^ x) & ~twos;
            twos = (twos ^ x) & ~ones;
        }
        return ones;
    }

    public static void main(String[] args) {
        System.out.println(singleNumber(new int[]{6, 1, 3, 3, 3, 6, 6}));
    }
}
