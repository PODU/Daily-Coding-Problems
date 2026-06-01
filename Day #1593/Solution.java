// rand5 from rand7 via rejection sampling: draw rand7, accept if <=5 else retry.
// Expected O(1) calls (acceptance prob 5/7). Output uniform on 1..5.
import java.util.Random;

public class Solution {
    static Random rng = new Random(12345); // deterministic seed

    // uniform 1..7 using language RNG
    static int rand7() {
        return rng.nextInt(7) + 1;
    }

    // uniform 1..5 via rejection sampling
    static int rand5() {
        while (true) {
            int v = rand7();
            if (v <= 5) return v;
        }
    }

    public static void main(String[] args) {
        final int N = 100000;
        int[] counts = new int[6];
        for (int i = 0; i < N; i++) counts[rand5()]++;

        System.out.println("Distribution over " + N + " samples:");
        for (int v = 1; v <= 5; v++)
            System.out.println("  " + v + ": " + counts[v]);

        StringBuilder sb = new StringBuilder("First 10 samples:");
        for (int i = 0; i < 10; i++) sb.append(" ").append(rand5());
        System.out.println(sb.toString());
    }
}
