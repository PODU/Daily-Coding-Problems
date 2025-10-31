// a+b = (a^b) + 2*(a&b). So c=(M-N)/2 must be a valid carry disjoint from N.
// Ordered pairs = 2^popcount(N), minus the two zero-cases when c==0. O(log M).
public class Solution {
    static long countPairs(long M, long N) {
        if (M < N || ((M - N) & 1) == 1) return 0;
        long c = (M - N) / 2;
        if ((c & N) != 0) return 0;
        long ways = 1L << Long.bitCount(N);
        if (c == 0) ways -= 2;
        return Math.max(ways, 0);
    }

    public static void main(String[] args) {
        System.out.println(countPairs(10, 4)); // 2
    }
}
