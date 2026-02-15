// Iterative Fibonacci with two rolling variables. O(n) time, O(1) space.
public class Solution {
    static long fib(int n) {
        if (n == 0) return 0;
        long a = 0, b = 1;
        for (int i = 2; i <= n; i++) { long c = a + b; a = b; b = c; }
        return b;
    }
    public static void main(String[] args) {
        StringBuilder sb = new StringBuilder();
        for (int i = 0; i <= 10; i++) sb.append(fib(i)).append(i < 10 ? " " : "");
        System.out.println(sb);
        System.out.println("fib(10) = " + fib(10));
    }
}
