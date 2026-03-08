// Day 1177: Find the element appearing once while all others appear 3 times.
// Track bits seen once (ones) and twice (twos); a third sighting clears both.
// Time O(N), Space O(1).
public class Solution {
    static int singleNumber(int[] a) {
        int ones = 0, twos = 0;
        for (int x : a) {
            ones = (ones ^ x) & ~twos;
            twos = (twos ^ x) & ~ones;
        }
        return ones;
    }

    public static void main(String[] args) {
        System.out.println(singleNumber(new int[]{6, 1, 3, 3, 3, 6, 6})); // 1
        System.out.println(singleNumber(new int[]{13, 19, 13, 13}));       // 19
    }
}
