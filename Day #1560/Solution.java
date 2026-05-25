// Greedy interval stabbing: sort by right endpoint; for the smallest uncovered right,
// pick point = max left among intervals containing it, cover them all. Time O(n^2), Space O(n).
import java.util.*;

public class Solution {
    static List<Integer> stabPoints(int[][] iv) {
        Arrays.sort(iv, (a, b) -> Integer.compare(a[1], b[1]));
        int n = iv.length;
        boolean[] covered = new boolean[n];
        List<Integer> points = new ArrayList<>();
        for (int i = 0; i < n; i++) {
            if (covered[i]) continue;
            int r = iv[i][1];
            int point = Integer.MIN_VALUE;
            for (int j = i; j < n; j++)
                if (!covered[j] && iv[j][0] <= r)
                    point = Math.max(point, iv[j][0]);
            points.add(point);
            for (int j = i; j < n; j++)
                if (!covered[j] && iv[j][0] <= point && point <= iv[j][1])
                    covered[j] = true;
        }
        return points;
    }

    public static void main(String[] args) {
        int[][] iv = {{0,3},{2,6},{3,4},{6,9}};
        List<Integer> pts = stabPoints(iv);
        StringBuilder sb = new StringBuilder("{");
        for (int i = 0; i < pts.size(); i++) {
            if (i > 0) sb.append(", ");
            sb.append(pts.get(i));
        }
        sb.append("}");
        System.out.println(sb);
    }
}
