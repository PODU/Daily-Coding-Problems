// Day 667: Simplified Elo rating. Expected score E = 1/(1+10^((Rb-Ra)/400)),
// update R += K*(actual - expected). Underdog gains more. Each update O(1).
import java.util.*;

public class Solution {
    static class Elo {
        double K = 32, start = 1200;
        Map<String, Double> r = new HashMap<>();
        double rating(String p) { return r.computeIfAbsent(p, k -> start); }
        void game(String winner, String loser) {
            double ra = rating(winner), rb = rating(loser);
            double ea = 1.0 / (1.0 + Math.pow(10, (rb - ra) / 400.0));
            double eb = 1.0 - ea;
            r.put(winner, ra + K * (1 - ea));
            r.put(loser, rb + K * (0 - eb));
        }
    }

    public static void main(String[] args) {
        Elo e = new Elo();
        e.r.put("A", 1200.0); e.r.put("B", 2000.0);
        e.game("A", "B");
        System.out.printf("A: %.1f%n", e.rating("A")); // ~1231.5
        System.out.printf("B: %.1f%n", e.rating("B")); // ~1968.5
    }
}
