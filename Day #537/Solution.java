// Collatz: iterate n->n/2 (even) or 3n+1 (odd) counting steps to 1; verify a range.
// Bonus longest under 1e6 via memoized step counts. Time ~O(LIMIT*avg-chain), space O(LIMIT).
public class Solution {
    static int steps(long n){ int c=0; while(n!=1){ n = (n%2==0)? n/2 : 3*n+1; c++; } return c; }

    public static void main(String[] args) {
        boolean allReach = true;
        for (int n=1; n<=20; n++) if (steps(n) < 0) allReach = false;
        System.out.println("Collatz conjecture holds for 1..20: " + allReach);

        final int LIMIT = 1000000;
        int[] dp = new int[LIMIT+1];
        int bestN = 1, bestLen = 0;
        for (int i=2; i<=LIMIT; i++) {
            long n = i; int c = 0;
            while (n != 1 && (n > LIMIT || dp[(int)n] == 0)) {
                n = (n%2==0)? n/2 : 3*n+1; c++;
            }
            c += (n==1)? 0 : dp[(int)n];
            dp[i] = c;
            if (c > bestLen) { bestLen = c; bestN = i; }
        }
        System.out.println("Longest under 1000000: n=" + bestN + " with " + bestLen + " steps");
    }
}
