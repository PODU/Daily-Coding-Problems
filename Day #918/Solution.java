// PageRank via power iteration. Dangling nodes distribute rank evenly.
// Time O(iters * (N+E)), Space O(N+E).
import java.util.*;

public class Solution {
    public static void main(String[] args) {
        int N = 4;
        int[][] out = {{1,2}, {2}, {0,1}, {0,1,2}};
        double d = 0.85;
        double[] rank = new double[N];
        Arrays.fill(rank, 1.0 / N);

        for (int iter = 0; iter < 1000; iter++) {
            double[] next = new double[N];
            for (int j = 0; j < N; j++) next[j] = (1.0 - d) / N;
            double dangling = 0.0;
            for (int i = 0; i < N; i++)
                if (out[i].length == 0) dangling += rank[i];
            for (int i = 0; i < N; i++) {
                if (out[i].length == 0) continue;
                double share = rank[i] / out[i].length;
                for (int j : out[i]) next[j] += d * share;
            }
            for (int j = 0; j < N; j++) next[j] += d * dangling / N;
            double diff = 0.0;
            for (int j = 0; j < N; j++) diff += Math.abs(next[j] - rank[j]);
            rank = next;
            if (diff < 1e-9) break;
        }
        for (int i = 0; i < N; i++)
            System.out.printf("%d: %.4f%n", i, rank[i]);
    }
}
