// Min squared integers summing to n via Lagrange four-square + Legendre three-square theorem.
// O(sqrt(n)) per query (only the i^2+j^2 check loops), O(1) space.
public class Solution {
    static boolean isPerfectSquare(long n) {
        long r = (long) Math.sqrt((double) n);
        while (r * r < n) r++;
        while (r * r > n) r--;
        return r * r == n;
    }

    static int numSquares(long n) {
        if (isPerfectSquare(n)) return 1;
        long m = n;
        while (m % 4 == 0) m /= 4;
        if (m % 8 == 7) return 4;
        for (long i = 1; i * i <= n; i++)
            if (isPerfectSquare(n - i * i)) return 2;
        return 3;
    }

    public static void main(String[] args) {
        System.out.println(numSquares(13)); // 2
        System.out.println(numSquares(27)); // 3
    }
}
