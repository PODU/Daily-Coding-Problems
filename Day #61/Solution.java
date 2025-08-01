// Fast (binary) exponentiation: square-and-multiply. Time O(log y), Space O(1).
public class Solution {
    static long fastPow(long x, long y) {
        long result = 1;
        if (y < 0) y = -y;
        while (y > 0) {
            if ((y & 1) == 1) result *= x;
            x *= x;
            y >>= 1;
        }
        return result;
    }

    public static void main(String[] args) {
        System.out.println(fastPow(2, 10)); // 1024
    }
}
