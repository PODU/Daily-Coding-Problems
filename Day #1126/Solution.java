// Smallest number of perfect squares summing to N.
// Legendre/Lagrange: 1 if perfect square; 4 if N=4^a(8b+7); 2 if sum of two squares; else 3. O(sqrt(N)) time, O(1) space.
public class Solution {
    static boolean isSquare(long n) {
        long r = (long) Math.sqrt((double) n);
        while (r * r < n) r++;
        while (r * r > n) r--;
        return r * r == n;
    }

    static int numSquares(long n) {
        if (isSquare(n)) return 1;
        long m = n;
        while (m % 4 == 0) m /= 4;
        if (m % 8 == 7) return 4;
        for (long a = 1; a * a <= n; a++) if (isSquare(n - a * a)) return 2;
        return 3;
    }

    public static void main(String[] args) {
        long N = 17;
        System.out.println(numSquares(N));
    }
}
