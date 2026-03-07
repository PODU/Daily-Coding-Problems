// Fast (binary) exponentiation by squaring on a 64-bit result.
// Time: O(log y), Space: O(1).
public class Solution {
    static long fastPow(long x, long y) {
        long result = 1;
        while (y > 0) {
            if ((y & 1) == 1) result *= x;
            x *= x;
            y >>= 1;
        }
        return result;
    }

    public static void main(String[] args) {
        System.out.println(fastPow(2, 10));
    }
}
