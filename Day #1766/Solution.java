// Merge overlapping intervals: sort by start, then sweep merging when the next
// start <= current end. Time: O(n log n), Space: O(n).
import java.util.ArrayList;
import java.util.Arrays;
import java.util.List;

public class Solution {
    static List<int[]> merge(int[][] iv) {
        Arrays.sort(iv, (a, b) -> Integer.compare(a[0], b[0]));
        List<int[]> res = new ArrayList<>();
        for (int[] p : iv) {
            if (!res.isEmpty() && p[0] <= res.get(res.size() - 1)[1])
                res.get(res.size() - 1)[1] = Math.max(res.get(res.size() - 1)[1], p[1]);
            else
                res.add(new int[]{p[0], p[1]});
        }
        return res;
    }

    public static void main(String[] args) {
        int[][] iv = {{1, 3}, {5, 8}, {4, 10}, {20, 25}};
        List<int[]> r = merge(iv);
        StringBuilder sb = new StringBuilder("[");
        for (int i = 0; i < r.size(); i++) {
            sb.append("(").append(r.get(i)[0]).append(", ").append(r.get(i)[1]).append(")");
            if (i + 1 < r.size()) sb.append(", ");
        }
        sb.append("]");
        System.out.println(sb.toString());
    }
}
