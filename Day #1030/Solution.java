// Day 1030: Quxes minimum remaining. Count colors; parity-based O(N) formula.
// If two counts are 0 -> n; else if all parities equal -> 2; else -> 1. Time O(N), Space O(1).
public class Solution {
    static int minQuxes(char[] q) {
        int r = 0, g = 0, b = 0;
        for (char c : q) {
            if (c == 'R') r++;
            else if (c == 'G') g++;
            else b++;
        }
        int n = r + g + b;
        int zeros = (r == 0 ? 1 : 0) + (g == 0 ? 1 : 0) + (b == 0 ? 1 : 0);
        if (zeros >= 2) return n;
        if ((r % 2 == g % 2) && (g % 2 == b % 2)) return 2;
        return 1;
    }

    public static void main(String[] args) {
        char[] q = {'R', 'G', 'B', 'G', 'B'};
        System.out.println(minQuxes(q));
    }
}
