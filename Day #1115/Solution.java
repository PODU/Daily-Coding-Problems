// Day 1115 - Uniform random in [0, n) excluding list l
// Approach: remap |E| excluded slots below m=n-|E| to available high slots,
// then sample once in [0, m). Time: O(|E|) prep, O(1)/sample.
import java.util.*;

public class Solution {
    static class Sampler {
        int n, m;
        Map<Integer, Integer> mapping = new HashMap<>();
        Random rng = new Random();

        Sampler(int n, int[] l) {
            this.n = n;
            TreeSet<Integer> excluded = new TreeSet<>();
            for (int x : l) if (x >= 0 && x < n) excluded.add(x);
            m = n - excluded.size();
            List<Integer> available = new ArrayList<>();
            for (int v = m; v < n; v++) if (!excluded.contains(v)) available.add(v);
            int ai = 0;
            for (int e : excluded) if (e < m) mapping.put(e, available.get(ai++));
        }

        int sample() {
            int r = rng.nextInt(m);
            return mapping.getOrDefault(r, r);
        }
    }

    public static void main(String[] args) {
        int n = 10; int[] l = {2, 5, 7};
        Sampler s = new Sampler(n, l);
        TreeSet<Integer> seen = new TreeSet<>();
        for (int i = 0; i < 2000; i++) seen.add(s.sample());
        System.out.println("n=10, excluded=[2, 5, 7]");
        System.out.println("Sampled valid numbers: " + seen);
    }
}
