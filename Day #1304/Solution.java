// Day 1304: Uniformly sample an integer in [0, n) not in list l.
// Precompute sorted excluded; pick r in [0, n-|excl|) and map to the r-th allowed value.
// Preprocess O(m log m); each draw O(m). Uniform over all allowed values.
import java.util.*;

public class Solution {
    static class RandExcluder {
        int n;
        int[] excl;
        Random rng;

        RandExcluder(int n, int[] l, long seed) {
            this.n = n;
            this.rng = new Random(seed);
            TreeSet<Integer> s = new TreeSet<>();
            for (int x : l) if (x >= 0 && x < n) s.add(x);
            excl = new int[s.size()];
            int i = 0;
            for (int v : s) excl[i++] = v;
        }

        int next() {
            int count = n - excl.length;
            int res = rng.nextInt(count);
            for (int e : excl) { if (e <= res) res++; else break; }
            return res;
        }
    }

    public static void main(String[] args) {
        RandExcluder r = new RandExcluder(5, new int[]{1, 3}, 42);
        TreeSet<Integer> seen = new TreeSet<>();
        for (int i = 0; i < 1000; i++) seen.add(r.next());
        System.out.println(seen); // [0, 2, 4]
    }
}
