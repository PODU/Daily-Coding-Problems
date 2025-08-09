// Day 88: Integer division using subtraction of shifted divisor (doubling).
// Time O(log^2 q), Space O(1).
public class Solution {
    static long divide(long dividend, long divisor) {
        long quotient = 0;
        while (dividend >= divisor) {
            long temp = divisor, multiple = 1;
            while (dividend >= (temp << 1)) { temp <<= 1; multiple <<= 1; }
            dividend -= temp;
            quotient += multiple;
        }
        return quotient;
    }

    public static void main(String[] args) {
        System.out.println(divide(10, 3)); // 3
        System.out.println(divide(27, 4)); // 6
    }
}
