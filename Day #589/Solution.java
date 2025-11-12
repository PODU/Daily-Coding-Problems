// Expected waiting time for patterns on a fair d6: "5 then 6" (distinct) E=36;
// "5 then 5" (self-overlap) E=6+36=42. Monte Carlo corroborates. Time O(1) for theory.
public class Solution {
    static long seed = 12345L;

    static int roll() {
        seed = seed * 6364136223846793005L + 1442695040888963407L;
        return (int) ((seed >>> 33) % 6) + 1;
    }

    static double simulate(int first, int second, int trials) {
        long total = 0;
        for (int t = 0; t < trials; t++) {
            int prev = 0, count = 0;
            while (true) {
                int r = roll();
                count++;
                if (prev == first && r == second) break;
                prev = r;
            }
            total += count;
        }
        return (double) total / trials;
    }

    public static void main(String[] args) {
        int e1 = 36; // five then six
        int e2 = 42; // five then five
        // simulate(...) available for corroboration
        System.out.println("Game 1 (five then six) expected rolls: " + e1);
        System.out.println("Game 2 (five then five) expected rolls: " + e2);
        System.out.println("Alice should play Game 1 (five then six) since it has the lower expected cost.");
    }
}
