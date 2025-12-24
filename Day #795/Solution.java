// Von Neumann debiasing: toss biased coin twice; (0,1)->0, (1,0)->1, else retry.
// Output is provably fair regardless of bias. O(1) expected tosses per fair bit.
public class Solution {
    private static long state = 123456789L;

    private static double nextRand() { // LCG in [0,1)
        state = state * 6364136223846793005L + 1442695040888963407L;
        return ((state >>> 11) & ((1L << 53) - 1)) / (double) (1L << 53);
    }

    static int tossBiased() { return nextRand() < 0.3 ? 1 : 0; } // P(1) = 0.3

    static int fairCoin() {
        while (true) {
            int a = tossBiased();
            int b = tossBiased();
            if (a == 0 && b == 1) return 0;
            if (a == 1 && b == 0) return 1;
        }
    }

    public static void main(String[] args) {
        final int N = 100000;
        int ones = 0;
        for (int i = 0; i < N; i++) ones += fairCoin();
        double p = (double) ones / N;
        System.out.printf("%.1f%n", p);
    }
}
