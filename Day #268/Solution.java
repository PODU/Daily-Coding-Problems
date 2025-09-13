// Day 268: Power of four check in O(1).
// Power of two (n & (n-1))==0 AND single bit at even position (n & 0x55555555). Time O(1), Space O(1).
public class Solution {
    static boolean isPowerOfFour(long n) {
        return n != 0 && (n & (n - 1)) == 0 && (n & 0x55555555L) != 0;
    }

    public static void main(String[] args) {
        long[] tests = {16, 8, 64, 5};
        for (long t : tests)
            System.out.println(t + " -> " + (isPowerOfFour(t) ? "True" : "False"));
    }
}
