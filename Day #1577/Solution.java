// Day 1577: Smallest sparse number (no adjacent 1 bits) >= N.
// Repeatedly fix the lowest pair of adjacent ones by clearing low bits and carrying.
// Time: O((log N)^2); Space: O(1).
public class Solution {
    static long smallestSparse(long n) {
        while ((n & (n >> 1)) != 0) {
            int i = 0;
            while (!(((n >> i) & 1L) != 0 && ((n >> (i + 1)) & 1L) != 0)) i++;
            long mask = (1L << (i + 2)) - 1;
            n = (n & ~mask) + (1L << (i + 2));
        }
        return n;
    }

    public static void main(String[] args) {
        System.out.println(smallestSparse(21)); // 21
    }
}
