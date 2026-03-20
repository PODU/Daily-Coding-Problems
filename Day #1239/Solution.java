// Integer division without * / %. Subtract doubled divisor. Time O(log^2 q).
public class Solution {
    static long divide(long dividend, long divisor) {
        long quotient = 0;
        while (dividend >= divisor) {
            long temp = divisor, multiple = 1;
            while ((temp << 1) <= dividend) { temp <<= 1; multiple <<= 1; }
            dividend -= temp;
            quotient += multiple;
        }
        return quotient;
    }

    public static void main(String[] args) {
        System.out.println(divide(43, 5));
        System.out.println(divide(100, 10));
    }
}
