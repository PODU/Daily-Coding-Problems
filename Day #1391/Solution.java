// Von Neumann fair coin from a biased toss: toss twice, (0,1)->0, (1,0)->1, else retry. O(1) expected per fair toss.
import java.util.Random;

public class Solution {
    static Random rng = new Random(12345);

    static int tossBiased() { return rng.nextDouble() < 0.3 ? 1 : 0; }

    static int fairToss() {
        while (true) {
            int a = tossBiased(), b = tossBiased();
            if (a == 0 && b == 1) return 0;
            if (a == 1 && b == 0) return 1;
        }
    }

    public static void main(String[] args) {
        int n = 100000, heads = 0;
        for (int i = 0; i < n; i++) heads += fairToss();
        System.out.printf("heads fraction: %.2f%n", (double) heads / n);
    }
}
