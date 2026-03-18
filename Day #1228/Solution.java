// Merge overlapping intervals: sort by start, sweep merging when start <= last end.
// Time: O(n log n), Space: O(n).
import java.util.*;

public class Solution {
    static List<int[]> merge(int[][] v) {
        Arrays.sort(v, (a, b) -> Integer.compare(a[0], b[0]));
        List<int[]> res = new ArrayList<>();
        for (int[] iv : v) {
            if (!res.isEmpty() && iv[0] <= res.get(res.size() - 1)[1])
                res.get(res.size() - 1)[1] = Math.max(res.get(res.size() - 1)[1], iv[1]);
            else
                res.add(new int[]{iv[0], iv[1]});
        }
        return res;
    }

    public static void main(String[] args) {
        int[][] in = {{1,3},{5,8},{4,10},{20,25}};
        List<int[]> out = merge(in);
        StringBuilder sb = new StringBuilder("[");
        for (int i = 0; i < out.size(); i++) {
            sb.append("(").append(out.get(i)[0]).append(", ").append(out.get(i)[1]).append(")");
            if (i + 1 < out.size()) sb.append(", ");
        }
        sb.append("]");
        System.out.println(sb.toString());
    }
}
