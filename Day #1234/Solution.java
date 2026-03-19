// Min Quxes remaining. One color -> N; all counts same parity -> 2; else 1.
// Time O(n), Space O(1).
public class Solution {
    static int minQuxes(String q) {
        if (q.isEmpty()) return 0;
        int r = 0, g = 0, b = 0;
        for (char c : q.toCharArray()) {
            if (c == 'R') r++; else if (c == 'G') g++; else b++;
        }
        int distinct = (r > 0 ? 1 : 0) + (g > 0 ? 1 : 0) + (b > 0 ? 1 : 0);
        if (distinct == 1) return q.length();
        if (r % 2 == g % 2 && g % 2 == b % 2) return 2;
        return 1;
    }

    public static void main(String[] args) {
        System.out.println(minQuxes("RGBGB"));
    }
}
