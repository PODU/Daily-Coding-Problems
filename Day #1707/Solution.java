// Elo rating: expected = 1/(1+10^((Rb-Ra)/400)); delta = K*(score-expected), zero-sum. O(1) per game.
import java.util.*;

public class Solution {
    static class EloSystem {
        Map<String, Double> ratings = new TreeMap<>();
        double K = 32.0;
        void add(String p) { ratings.put(p, 1200.0); }
        static double expected(double ra, double rb) {
            return 1.0 / (1.0 + Math.pow(10.0, (rb - ra) / 400.0));
        }
        void recordGame(String w, String l) {
            double rw = ratings.get(w), rl = ratings.get(l);
            double ew = expected(rw, rl);
            double delta = K * (1.0 - ew);
            ratings.put(w, rw + delta);
            ratings.put(l, rl - delta);
            System.out.println(w + " beats " + l + ": "
                + w + " " + Math.round(rw) + "->" + Math.round(rw + delta) + ", "
                + l + " " + Math.round(rl) + "->" + Math.round(rl - delta));
        }
    }

    public static void main(String[] args) {
        EloSystem e = new EloSystem();
        e.add("A"); e.add("B"); e.add("C"); e.add("D");
        e.recordGame("A", "B");
        e.recordGame("A", "C");
        e.recordGame("D", "A");
        System.out.println("Final ratings:");
        for (Map.Entry<String, Double> p : e.ratings.entrySet())
            System.out.println(p.getKey() + " " + Math.round(p.getValue()));
    }
}
