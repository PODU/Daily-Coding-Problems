// Day 1730: Fast integer exponentiation (exponentiation by squaring).
// Square the base and halve the exponent each step. Time: O(log y). Space: O(1).
public class Solution {
    static long fastPow(long x, long y) {
        if (y < 0) { // x^(-y) = 1 / x^y; integer result only for x == 1 or -1.
            long inv = fastPow(x, -y);
            return inv == 0 ? 0 : 1 / inv;
        }
        long result = 1, base = x;
        while (y > 0) {
            if ((y & 1) == 1) result *= base;
            base *= base;
            y >>= 1;
        }
        return result;
    }

    public static void main(String[] args) {
        System.out.println(fastPow(2, 10)); // 1024
    }
}
