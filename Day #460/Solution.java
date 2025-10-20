// Day 460: Min flips so every 'x' precedes every 'y'.
// One-pass DP: dp = min(flip this x->y, flip all prior y->x). Time O(n), Space O(1).
public class Solution {
    static int minFlips(String s) {
        int dp = 0, y = 0;
        for (char c : s.toCharArray()) {
            if (c == 'y') y++;
            else dp = Math.min(dp + 1, y);
        }
        return dp;
    }

    public static void main(String[] args) {
        System.out.println(minFlips("xyxxxyxyy")); // 2
    }
}
