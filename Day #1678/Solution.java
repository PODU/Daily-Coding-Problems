// Day 1678: Integer division without / * %. Subtract largest shifted multiple of
// divisor each round (doubling). Time O(log(quotient)), Space O(1).
public class Solution {
    static long divide(long a, long b) {
        long q = 0;
        while (a >= b) {
            long temp = b, mult = 1;
            while (a >= (temp << 1)) { temp <<= 1; mult <<= 1; }
            a -= temp; q += mult;
        }
        return q;
    }

    public static void main(String[] args) {
        System.out.println(divide(43, 8)); // 5
    }
}
