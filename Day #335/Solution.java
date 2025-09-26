// PageRank via iterative power method, d=0.85. Iterate until convergence.
// Time: O(iters * (N + E)). Space: O(N + E).
import java.util.*;

public class Solution {
    public static void main(String[] args) {
        String[] names = {"A", "B", "C", "D"};
        int N = names.length;
        Map<String,Integer> idx = new HashMap<>();
        for (int i = 0; i < N; i++) idx.put(names[i], i);
        String[][] edges = {{"A","B"},{"A","C"},{"B","C"},{"C","A"},{"D","C"}};
        List<List<Integer>> in = new ArrayList<>();
        for (int i = 0; i < N; i++) in.add(new ArrayList<>());
        int[] out = new int[N];
        for (String[] e : edges) { in.get(idx.get(e[1])).add(idx.get(e[0])); out[idx.get(e[0])]++; }

        double d = 0.85;
        double[] score = new double[N];
        Arrays.fill(score, 1.0 / N);
        for (int it = 0; it < 1000; it++) {
            double[] nxt = new double[N];
            Arrays.fill(nxt, (1.0 - d) / N);
            for (int j = 0; j < N; j++)
                for (int i : in.get(j))
                    nxt[j] += d * score[i] / out[i];
            double diff = 0;
            for (int k = 0; k < N; k++) diff += Math.abs(nxt[k] - score[k]);
            score = nxt;
            if (diff < 1e-9) break;
        }
        for (int i = 0; i < N; i++)
            System.out.printf("%s: %.4f%n", names[i], score[i]);
    }
}
