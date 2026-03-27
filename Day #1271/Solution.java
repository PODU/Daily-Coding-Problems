// Day 1271: Implement rand5() from rand7() with uniform probability.
// Rejection sampling: redraw rand7 until result <= 5. Expected O(7/5) calls per sample.
import java.util.*;

public class Solution {
    static final Random RNG = new Random();

    static int rand7() { return RNG.nextInt(7) + 1; }

    static int rand5() {
        int v;
        do { v = rand7(); } while (v > 5); // reject 6,7 -> uniform 1..5
        return v;
    }

    public static void main(String[] args) {
        int[] count = new int[6];
        int trials = 100000;
        for (int i = 0; i < trials; i++) count[rand5()]++;
        System.out.println("Distribution over " + trials + " samples (expect ~" + trials / 5 + " each):");
        for (int v = 1; v <= 5; v++) System.out.println(v + ": " + count[v]);
    }
}
