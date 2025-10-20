// Square root via Newton's method: x = (x + n/x)/2 until convergence.
// Time: O(log(1/eps)) iterations, Space: O(1).
public class Solution {
    static double mySqrt(double n) {
        if (n < 0) return Double.NaN;
        if (n == 0) return 0;
        double x = n;
        for (int i = 0; i < 100; i++) {
            double nx = 0.5 * (x + n / x);
            if (Math.abs(nx - x) < 1e-12) break;
            x = nx;
        }
        return x;
    }

    public static void main(String[] args) {
        double n = 9;
        double r = mySqrt(n);
        if (Math.abs(r - Math.round(r)) < 1e-9)
            System.out.println(Math.round(r));
        else
            System.out.println(r);
    }
}
