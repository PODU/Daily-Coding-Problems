// Integer division without / * %: subtract largest shifted multiple of divisor.
// Bit-shifting. Time O((log n)^2), Space O(1).
public class Solution {
    static long divide(long dividend, long divisor) {
        long quotient = 0;
        while (dividend >= divisor) {
            long temp = divisor, multiple = 1;
            while (dividend >= (temp << 1)) {
                temp <<= 1;
                multiple <<= 1;
            }
            dividend -= temp;
            quotient += multiple;
        }
        return quotient;
    }

    public static void main(String[] args) {
        System.out.println(divide(43, 8));
        System.out.println(divide(100, 9));
    }
}
