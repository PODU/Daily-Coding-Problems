// Day 1850: rand7() from rand5() via rejection sampling on the 1..25 grid.
// Expected O(1) amortized calls (acceptance 21/25); uniform over 1..7.
import java.util.*;

public class Solution {
    static Random rng = new Random(12345);
    static int rand5() { return rng.nextInt(5) + 1; }

    static int rand7() {
        while (true) {
            int v = 5 * (rand5() - 1) + rand5(); // 1..25
            if (v <= 21) return (v - 1) % 7 + 1;
        }
    }

    public static void main(String[] args) {
        int[] counts = new int[8];
        for (int i = 0; i < 70000; i++) counts[rand7()]++;
        StringBuilder sb = new StringBuilder("Sample of 10:");
        for (int i = 0; i < 10; i++) sb.append(" ").append(rand7());
        System.out.println(sb);
        System.out.println("Histogram over 70000 draws (each ~10000):");
        for (int i = 1; i <= 7; i++) System.out.println("  " + i + ": " + counts[i]);
    }
}
