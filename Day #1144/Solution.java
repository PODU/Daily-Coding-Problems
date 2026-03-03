// Day 1144: count positive pairs (a,b) with a+b=M, a^b=N.
// a+b = (a^b) + 2*(a&b) => and=(M-N)/2; valid iff M>=N, even, and&N==0.
// Ordered pairs = 2^popcount(N), minus 2 if and==0 (excludes a=0/b=0). O(1).
public class Solution {
    static long countPairs(long M, long N) {
        if (M < N || ((M - N) & 1) == 1) return 0;
        long aAnd = (M - N) / 2;
        if ((aAnd & N) != 0) return 0;
        long cnt = 1L << Long.bitCount(N);
        if (aAnd == 0) cnt -= 2;
        return cnt < 0 ? 0 : cnt;
    }

    public static void main(String[] args) {
        System.out.println(countPairs(10, 4)); // 2 -> (7,3) and (3,7)
    }
}
