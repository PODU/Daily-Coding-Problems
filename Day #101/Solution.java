// Day 101: Goldbach. Sieve primes up to n, then the smallest prime a with n-a
// prime gives the lexicographically smallest pair. O(n log log n).
public class Solution {
    static int[] goldbach(int n) {
        boolean[] composite = new boolean[n + 1];
        for (int i = 2; (long) i * i <= n; i++)
            if (!composite[i])
                for (int j = i * i; j <= n; j += i) composite[j] = true;
        for (int a = 2; a <= n / 2; a++)
            if (!composite[a] && !composite[n - a]) return new int[]{a, n - a};
        return null;
    }

    public static void main(String[] args) {
        int[] p = goldbach(4);
        System.out.println(p[0] + " + " + p[1] + " = " + (p[0] + p[1])); // 2 + 2 = 4
    }
}
