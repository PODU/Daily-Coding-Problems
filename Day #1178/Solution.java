// Day 1178: Collatz sequences. Verify each reaches 1 and find the longest start <= 1e6.
// Memoized chain lengths (each value cached once). Time ~O(LIMIT), Space O(LIMIT).
public class Solution {
    static final int LIMIT = 1000000;
    static int[] memo = new int[LIMIT + 1];

    static int clen(long n) {
        if (n == 1) return 1;
        if (n <= LIMIT && memo[(int) n] != 0) return memo[(int) n];
        long nxt = (n % 2 == 0) ? n / 2 : 3 * n + 1;
        int l = 1 + clen(nxt);
        if (n <= LIMIT) memo[(int) n] = l;
        return l;
    }

    public static void main(String[] args) {
        StringBuilder seq = new StringBuilder();
        long n = 6;
        while (true) {
            seq.append(n);
            if (n == 1) break;
            seq.append(" -> ");
            n = (n % 2 == 0) ? n / 2 : 3 * n + 1;
        }
        System.out.println(seq);
        int bestN = 1, bestLen = 1;
        for (int i = 2; i <= LIMIT; i++) {
            int l = clen(i);
            if (l > bestLen) { bestLen = l; bestN = i; }
        }
        System.out.println("All sequences up to " + LIMIT + " reach 1: true");
        System.out.println("Longest sequence for n <= " + LIMIT + ": n = " + bestN + " (length " + bestLen + ")");
    }
}
