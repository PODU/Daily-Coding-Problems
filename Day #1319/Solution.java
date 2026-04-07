// Square root of a real number via Newton's method: x <- (x + n/x)/2.
// Quadratic convergence -> O(log(1/eps)) iterations, O(1) space.
public class Solution {
    static double mySqrt(double n) {
        if (n < 0) throw new IllegalArgumentException("negative");
        if (n == 0) return 0;
        double x = n;
        for (int i = 0; i < 100; i++) {
            double nx = 0.5 * (x + n / x);
            if (Math.abs(nx - x) < 1e-15) break;
            x = nx;
        }
        return x;
    }

    public static void main(String[] args) {
        double r = mySqrt(9);
        if (Math.abs(r - Math.round(r)) < 1e-9) System.out.println(Math.round(r)); // 3
        else System.out.println(r);
    }
}
