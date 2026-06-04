// Goldbach pair: sieve up to n, scan a from 2; first a where a and n-a prime.
// Smallest a => lexicographically smallest [a,b]. Time O(n log log n), Space O(n).
public class Solution {
    static int[] goldbach(int n) {
        boolean[] isPrime = new boolean[n + 1];
        java.util.Arrays.fill(isPrime, true);
        isPrime[0] = isPrime[1] = false;
        for (int i = 2; (long) i * i <= n; i++)
            if (isPrime[i])
                for (int j = i * i; j <= n; j += i) isPrime[j] = false;
        for (int a = 2; a <= n - a; a++)
            if (isPrime[a] && isPrime[n - a]) return new int[]{a, n - a};
        return new int[]{-1, -1};
    }

    public static void main(String[] args) {
        int n = 4;
        int[] r = goldbach(n);
        System.out.println(r[0] + " + " + r[1] + " = " + n);
    }
}
