// Square root via Newton's method: x = (x + n/x)/2, quadratic convergence.
// Time: O(log(1/eps)) iterations, Space: O(1).
public class Solution {
    static double mySqrt(double n) {
        if (n < 0) return Double.NaN;
        if (n == 0) return 0;
        double x = n;
        for (int i = 0; i < 100; i++) {
            double nx = 0.5 * (x + n / x);
            if (Math.abs(nx - x) < 1e-15) { x = nx; break; }
            x = nx;
        }
        return x;
    }

    static void printResult(double n) {
        double r = mySqrt(n);
        long ri = Math.round(r);
        if (Math.abs(r - ri) < 1e-9) System.out.println(ri);            // exact integer
        else System.out.printf("%.8f%n", r);
    }

    public static void main(String[] args) {
        printResult(9);   // -> 3
        printResult(2);   // -> 1.41421356
    }
}
