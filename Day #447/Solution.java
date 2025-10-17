// Day 447: Integer pow via exponentiation by squaring. O(log y) time, O(1) space.
public class Solution {
    // returns x^y; negative y returns a double
    static double ipow(long x, long y) {
        if (y < 0) return 1.0 / ipow(x, -y);
        long result = 1, base = x;
        while (y > 0) {
            if ((y & 1) == 1) result *= base;
            base *= base;
            y >>= 1;
        }
        return (double) result;
    }

    public static void main(String[] args) {
        System.out.println((long) ipow(2, 10)); // 1024
        System.out.println((long) ipow(3, 5));  // 243
        System.out.println(ipow(2, -2));         // 0.25
    }
}
