// Rearrange string so no two adjacent chars equal.
// Greedy: place chars by descending frequency into even indices then odd indices.
// Time O(n + k log k), Space O(n).
import java.util.*;

public class Solution {
    static String reorganize(String s) {
        int n = s.length();
        int[] cnt = new int[256];
        for (char c : s.toCharArray()) cnt[c]++;
        int maxc = 0;
        for (int v : cnt) maxc = Math.max(maxc, v);
        if (maxc > (n + 1) / 2) return null;
        List<int[]> list = new ArrayList<>();
        for (int i = 0; i < 256; i++) if (cnt[i] > 0) list.add(new int[]{cnt[i], i});
        list.sort((a, b) -> b[0] - a[0]);
        char[] res = new char[n];
        int idx = 0;
        for (int[] p : list)
            for (int j = 0; j < p[0]; j++) {
                res[idx] = (char) p[1];
                idx += 2;
                if (idx >= n) idx = 1;
            }
        return new String(res);
    }

    public static void main(String[] args) {
        String r = reorganize("aaabbc");
        System.out.println(r == null ? "None" : r);
    }
}
