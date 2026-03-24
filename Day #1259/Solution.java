// Min flips so all x before all y: single-pass DP. flips=min(flips+1, yCount) on 'x', yCount++ on 'y'. O(n) time, O(1) space.
public class Solution {
    static int minFlips(String s) {
        int flips = 0, yCount = 0;
        for (char c : s.toCharArray()) {
            if (c == 'y') yCount++;
            else flips = Math.min(flips + 1, yCount);
        }
        return flips;
    }

    public static void main(String[] args) {
        System.out.println(minFlips("xyxxxyxyy"));
    }
}
