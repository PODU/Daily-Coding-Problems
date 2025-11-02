// Egg drop (min worst-case trials): find smallest m such that with N eggs we can
// cover k floors. f(m, N) = sum_{i=1..N} C(m,i); smallest m with f(m,N) >= k.
// Time: O(m * N) where m is the answer; Space: O(1).
public class Solution {
    static int eggDrop(long N, long k) {
        int m = 0;
        long covered = 0;
        while (covered < k) {
            m++;
            long sum = 0, term = 1; // term = C(m, i)
            for (int i = 1; i <= N; i++) {
                term = term * (m - i + 1) / i; // C(m,i)
                sum += term;
                if (sum >= k) break;
            }
            covered = sum;
        }
        return m;
    }

    public static void main(String[] args) {
        System.out.println(eggDrop(1, 5)); // expected 5
    }
}
