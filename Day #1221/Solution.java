// Iterative rolling Fibonacci: two variables, O(n) time, O(1) space.
// fib(0)=0, fib(1)=1.
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
        System.out.println(fib(10));
    }
}
