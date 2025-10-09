// Greedy activity selection: sort by end time, pick job if start >= last end (intervals [start,end)).
// Time O(n log n), Space O(n).
import java.util.*;

public class Solution {
    static List<int[]> selectJobs(int[][] jobs) {
        int[][] sorted = jobs.clone();
        Arrays.sort(sorted, (a, b) -> Integer.compare(a[1], b[1]));
        List<int[]> chosen = new ArrayList<>();
        int lastEnd = Integer.MIN_VALUE;
        for (int[] j : sorted) {
            if (j[0] >= lastEnd) {
                chosen.add(j);
                lastEnd = j[1];
            }
        }
        return chosen;
    }

    public static void main(String[] args) {
        int[][] jobs = {{0,6},{1,4},{3,5},{3,8},{4,7},{5,9},{6,10},{8,11}};
        List<int[]> chosen = selectJobs(jobs);
        StringBuilder sb = new StringBuilder("[");
        for (int i = 0; i < chosen.size(); i++) {
            int[] j = chosen.get(i);
            sb.append("(").append(j[0]).append(", ").append(j[1]).append(")");
            if (i + 1 < chosen.size()) sb.append(", ");
        }
        sb.append("]");
        System.out.println(sb.toString());
    }
}
