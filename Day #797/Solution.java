// Day 797: Goldbach - two primes summing to even n, lexicographically smallest.
// Scan a from 2 upward; first prime a with prime (n-a) gives smallest pair.
// Time O(n * sqrt(n)), Space O(1).
public class Solution {
    static boolean isPrime(int x) {
        if (x < 2) return false;
        for (int d = 2; (long) d * d <= x; d++)
            if (x % d == 0) return false;
        return true;
    }

    static int[] goldbach(int n) {
        for (int a = 2; a <= n / 2; a++)
            if (isPrime(a) && isPrime(n - a)) return new int[]{a, n - a};
        return new int[]{-1, -1};
    }

    public static void main(String[] args) {
        int n = 4;
        int[] r = goldbach(n);
        System.out.println(r[0] + " + " + r[1] + " = " + n); // 2 + 2 = 4
    }
}
