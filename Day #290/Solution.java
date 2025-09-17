// Quxes: adjacent different colors merge to third. Smallest remaining count.
// Count r,g,b; distinct<=1 -> total; all same parity -> 2; else 1. Time O(n), Space O(1).
public class Solution {
    static int smallestQuxes(char[] q) {
        int r = 0, g = 0, b = 0;
        for (char c : q) {
            if (c == 'R') r++;
            else if (c == 'G') g++;
            else b++;
        }
        int distinct = (r > 0 ? 1 : 0) + (g > 0 ? 1 : 0) + (b > 0 ? 1 : 0);
        if (distinct <= 1) return r + g + b;
        if (r % 2 == g % 2 && g % 2 == b % 2) return 2;
        return 1;
    }

    public static void main(String[] args) {
        char[] q = {'R', 'G', 'B', 'G', 'B'};
        System.out.println(smallestQuxes(q));
    }
}
