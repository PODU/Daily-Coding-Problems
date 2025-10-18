// Day 451: nth Fibonacci number in O(1) space.
// Iterative rolling pair. Time O(n), Space O(1).
public class Solution {
    static long fib(int n) {
        if (n < 2) return n;
        long a = 0, b = 1;
        for (int i = 2; i <= n; i++) {
            long c = a + b;
            a = b;
            b = c;
        }
        return b;
    }

    public static void main(String[] args) {
        System.out.println(fib(10)); // 55
    }
}
