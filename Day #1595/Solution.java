// Approach: DP min palindrome partition. isPal[i][j] table O(n^2), cut[i]=min cuts for prefix,
// then reconstruct one optimal partition. Time O(n^2), Space O(n^2).
import java.util.*;

public class Solution {
    static List<String> minPalPartition(String s) {
        int n = s.length();
        List<String> res = new ArrayList<>();
        if (n == 0) return res;
        boolean[][] pal = new boolean[n][n];
        for (int i = 0; i < n; i++) pal[i][i] = true;
        for (int len = 2; len <= n; len++)
            for (int i = 0; i + len - 1 < n; i++) {
                int j = i + len - 1;
                if (s.charAt(i) == s.charAt(j) && (len == 2 || pal[i+1][j-1])) pal[i][j] = true;
            }
        int[] cut = new int[n];
        int[] start = new int[n];
        for (int i = 0; i < n; i++) {
            if (pal[0][i]) { cut[i] = 0; start[i] = 0; continue; }
            int best = Integer.MAX_VALUE, bj = 0;
            for (int j = 1; j <= i; j++)
                if (pal[j][i] && cut[j-1] + 1 < best) { best = cut[j-1] + 1; bj = j; }
            cut[i] = best; start[i] = bj;
        }
        int i = n - 1;
        while (i >= 0) { int j = start[i]; res.add(s.substring(j, i + 1)); i = j - 1; }
        Collections.reverse(res);
        return res;
    }

    static void printList(List<String> v) {
        StringBuilder sb = new StringBuilder("[");
        for (int i = 0; i < v.size(); i++) { sb.append("\"").append(v.get(i)).append("\""); if (i + 1 < v.size()) sb.append(", "); }
        sb.append("]");
        System.out.println(sb.toString());
    }

    public static void main(String[] args) {
        printList(minPalPartition("racecarannakayak"));
        printList(minPalPartition("abc"));
    }
}
