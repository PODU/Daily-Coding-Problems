// Merge overlapping intervals: sort by start, merge when next.start <= current.end.
// Time: O(n log n), Space: O(n).
import java.util.*;

public class Solution {
    public static void main(String[] args) {
        int[][] iv = {{1,3},{5,8},{4,10},{20,25}};
        Arrays.sort(iv, (a, b) -> Integer.compare(a[0], b[0]));
        List<int[]> res = new ArrayList<>();
        for (int[] p : iv) {
            if (!res.isEmpty() && p[0] <= res.get(res.size()-1)[1])
                res.get(res.size()-1)[1] = Math.max(res.get(res.size()-1)[1], p[1]);
            else
                res.add(new int[]{p[0], p[1]});
        }
        StringBuilder sb = new StringBuilder("[");
        for (int i = 0; i < res.size(); i++) {
            sb.append("(").append(res.get(i)[0]).append(", ").append(res.get(i)[1]).append(")");
            if (i + 1 < res.size()) sb.append(", ");
        }
        sb.append("]");
        System.out.println(sb.toString());
    }
}
