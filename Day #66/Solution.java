// Von Neumann extractor: toss biased coin twice; (0,1)->0, (1,0)->1, else retry => 50/50. Time O(1) expected.
import java.util.Random;

public class Solution {
    static final Random rng = new Random(12345);

    static int tossBiased() { // simulated bias p = 0.3 for 1
        return rng.nextDouble() < 0.3 ? 1 : 0;
    }

    static int tossFair() {
        while (true) {
            int a = tossBiased();
            int b = tossBiased();
            if (a == 0 && b == 1) return 0;
            if (a == 1 && b == 0) return 1;
        }
    }

    public static void main(String[] args) {
        int trials = 100000;
        long ones = 0;
        for (int i = 0; i < trials; i++) ones += tossFair();
        double frac = (double) ones / trials;
        assert frac > 0.48 && frac < 0.52;
        if (!(frac > 0.48 && frac < 0.52)) throw new RuntimeException("not fair");
        System.out.println("Fair coin ~0.5");
    }
}
