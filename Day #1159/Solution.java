// Goldbach: iterate a from 2 upward, return first (a, n-a) both prime (lexicographically smallest).
// Time: O(n*sqrt(n)) worst, Space: O(1).
public class Solution {
    static boolean isPrime(long x) {
        if (x < 2) return false;
        for (long i = 2; i * i <= x; i++)
            if (x % i == 0) return false;
        return true;
    }

    static long[] goldbach(long n) {
        for (long a = 2; a <= n / 2; a++)
            if (isPrime(a) && isPrime(n - a)) return new long[]{a, n - a};
        return new long[]{-1, -1};
    }

    public static void main(String[] args) {
        long n = 4;
        long[] p = goldbach(n);
        System.out.println(p[0] + " + " + p[1] + " = " + n);
    }
}
