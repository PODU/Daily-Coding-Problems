// Bitwise AND of all integers in [M, N] = common binary prefix of M and N.
// Shift both right until equal, then shift back. Time O(log N), Space O(1).
public class Solution {
    static long rangeAnd(long m, long n) {
        int shift = 0;
        while (m < n) { m >>= 1; n >>= 1; shift++; }
        return m << shift;
    }

    public static void main(String[] args) {
        System.out.println(rangeAnd(5, 7)); // 4  (5 & 6 & 7)
    }
}
