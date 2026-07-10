// fib(n): iterative two-variable rolling sum.
// Time O(N), Space O(1).
public class Solution {
    static long fib(int n) {
        long a = 0, b = 1;
        for (int i = 0; i < n; i++) {
            long t = a + b;
            a = b;
            b = t;
        }
        return a;
    }
    public static void main(String[] args) {
        System.out.println(fib(10));
    }
}
