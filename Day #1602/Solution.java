// Min flips so all x precede all y. DP: at each char, flips = min(yCount, flips+1):
// flipping current 'y'->'x' costs all prior y's; flipping current 'x'->'y' costs flips+1. Time O(n), space O(1).
public class Solution {
    static int minFlips(String s) {
        int flips = 0, y = 0;
        for (int i = 0; i < s.length(); i++) {
            char c = s.charAt(i);
            if (c == 'y') y++;
            else flips = Math.min(y, flips + 1);
        }
        return flips;
    }

    public static void main(String[] args) {
        System.out.println(minFlips("xyxxxyxyy")); // 2
    }
}
