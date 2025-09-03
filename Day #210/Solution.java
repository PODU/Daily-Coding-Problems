// Day 210: Collatz conjecture - verify reach to 1 and find longest sequence for n <= 1e6.
// Memoized sequence lengths (cache for values <= LIMIT). Time: ~O(LIMIT * avg steps), Space: O(LIMIT).
import java.util.*;

public class Solution {
    static final int LIMIT = 1000000;
    static int[] cache = new int[LIMIT + 1];

    static int collatzLen(long start) {
        ArrayList<Long> path = new ArrayList<>();
        long m = start;
        while (m > LIMIT || cache[(int) m] == 0) {
            path.add(m);
            m = (m % 2 == 0) ? m / 2 : 3 * m + 1;
        }
        int base = cache[(int) m];
        for (int i = path.size() - 1; i >= 0; i--) {
            base++;
            long v = path.get(i);
            if (v <= LIMIT) cache[(int) v] = base;
        }
        return base;
    }

    public static void main(String[] args) {
        cache[1] = 1;
        System.out.println("Collatz length of 27: " + collatzLen(27)); // 112
        int bestN = 1, bestLen = 1;
        for (int n = 1; n <= LIMIT; n++) {
            int l = collatzLen(n);
            if (l > bestLen) { bestLen = l; bestN = n; }
        }
        System.out.println("Longest sequence for n <= 1000000: n=" + bestN
                + " (length " + bestLen + ")"); // n=837799 (length 525)
    }
}
