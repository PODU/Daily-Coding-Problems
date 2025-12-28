// Simplified Elo rating system. Expected score logistic, K=32 update on win/loss.
// recordGame adjusts both players. Time O(1) per game, Space O(players).
import java.util.*;

public class Solution {
    static final double K = 32.0;
    Map<String, Double> ratings = new HashMap<>();

    void add(String name) { ratings.put(name, 1200.0); }
    void add(String name, double r) { ratings.put(name, r); }

    double expected(double ra, double rb) {
        return 1.0 / (1.0 + Math.pow(10.0, (rb - ra) / 400.0));
    }

    void recordGame(String winner, String loser) {
        double ra = ratings.get(winner), rb = ratings.get(loser);
        double ea = expected(ra, rb), eb = expected(rb, ra);
        ratings.put(winner, ra + K * (1.0 - ea));
        ratings.put(loser, rb + K * (0.0 - eb));
    }

    public static void main(String[] args) {
        Solution e = new Solution();
        e.add("A"); e.add("B");
        System.out.printf("Initial: A=%.2f B=%.2f%n", e.ratings.get("A"), e.ratings.get("B"));
        e.recordGame("B", "A");
        System.out.printf("After B beats A (equal): A=%.2f B=%.2f%n", e.ratings.get("A"), e.ratings.get("B"));

        Solution e2 = new Solution();
        e2.add("C", 1000.0); e2.add("D", 1600.0);
        System.out.printf("Initial: C=%.2f D=%.2f%n", e2.ratings.get("C"), e2.ratings.get("D"));
        e2.recordGame("C", "D");
        System.out.printf("After underdog C beats D: C=%.2f D=%.2f%n", e2.ratings.get("C"), e2.ratings.get("D"));
        System.out.printf("Underdog gained %.2f vs even-match gain %.2f%n",
                e2.ratings.get("C") - 1000.0, e.ratings.get("B") - 1200.0);
    }
}
