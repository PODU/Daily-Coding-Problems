// Count ordered (a,b), a+b=M, a^b=N. c=(M-N)/2; valid if c&N==0.
// Count=2^popcount(N), minus 2 if M==N. Time O(1), Space O(1).
public class Solution {
    static long countPairs(long M, long N) {
        long diff = M - N;
        if (diff < 0 || (diff & 1) == 1) return 0;
        long c = diff / 2;
        if ((c & N) != 0) return 0;
        long count = 1L << Long.bitCount(N);
        if (M == N) count -= 2;
        return count < 0 ? 0 : count;
    }

    public static void main(String[] args) {
        System.out.println(countPairs(10, 4));
    }
}
