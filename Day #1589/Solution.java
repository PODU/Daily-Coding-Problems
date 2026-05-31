// Quxes minimization: count R/G/B; two zero counts -> n; all parities equal -> 2; else 1.
// Time O(n), Space O(1).
public class Solution {
    static int minQuxes(char[] a) {
        int r = 0, g = 0, b = 0;
        for (char c : a) {
            if (c == 'R') r++;
            else if (c == 'G') g++;
            else b++;
        }
        int n = a.length;
        int zeros = (r == 0 ? 1 : 0) + (g == 0 ? 1 : 0) + (b == 0 ? 1 : 0);
        if (zeros >= 2) return n;
        if ((r % 2) == (g % 2) && (g % 2) == (b % 2)) return 2;
        return 1;
    }

    public static void main(String[] args) {
        char[] demo = {'R', 'G', 'B', 'G', 'B'};
        System.out.println(minQuxes(demo));
    }
}
