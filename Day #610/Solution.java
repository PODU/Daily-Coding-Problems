// Day 610: Integer division of positive ints without / , * , or %.
// Approach: repeated doubling subtraction (binary long division). Time O(log^2), Space O(1).
public class Solution {
    static long divide(long dividend, long divisor) {
        long q = 0;
        while (dividend >= divisor) {
            long temp = divisor, mult = 1;
            while (dividend >= (temp << 1)) { temp <<= 1; mult <<= 1; }
            dividend -= temp;
            q += mult;
        }
        return q;
    }

    public static void main(String[] args) {
        System.out.println(divide(10, 3)); // 3
        System.out.println(divide(43, 8)); // 5
    }
}
