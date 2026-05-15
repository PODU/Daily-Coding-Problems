// Single number among triples: bitwise ones/twos accumulators isolate the unique value.
// Time: O(n). Space: O(1).
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
        int[] a = {6, 1, 3, 3, 3, 6, 6};
        int[] b = {13, 19, 13, 13};
        System.out.println(singleNumber(a)); // 1
        System.out.println(singleNumber(b)); // 19
    }
}
