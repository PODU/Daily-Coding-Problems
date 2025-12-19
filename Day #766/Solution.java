// Day 766: Minimum flips so all 'x' come before all 'y'.
// One-pass DP: flips = min(flips+1, countY). O(n) time, O(1) space.
public class Solution {
    static int minFlips(String s) {
        int flips = 0, countY = 0;
        for (char c : s.toCharArray()) {
            if (c == 'y') countY++;
            else flips = Math.min(flips + 1, countY);
        }
        return flips;
    }

    public static void main(String[] args) {
        System.out.println(minFlips("xyxxxyxyy")); // 2
    }
}
