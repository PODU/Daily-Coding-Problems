// Day 449: Goldbach pair. Scan a from 2 upward; the first a where a and n-a are
// both prime gives the lexicographically smallest pair. O(n*sqrt(n)) worst case.
public class Solution {
    static boolean isPrime(int x) {
        if (x < 2) return false;
        for (int i = 2; (long) i * i <= x; i++) if (x % i == 0) return false;
        return true;
    }

    static int[] goldbach(int n) {
        for (int a = 2; a <= n / 2; a++)
            if (isPrime(a) && isPrime(n - a)) return new int[]{a, n - a};
        return new int[]{-1, -1};
    }

    public static void main(String[] args) {
        int n = 4;
        int[] p = goldbach(n);
        System.out.println(p[0] + " + " + p[1] + " = " + n); // 2 + 2 = 4
    }
}
