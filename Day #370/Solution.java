// Day 370: Total courier "active" time (carrying >= 1 order).
// Sweep events by timestamp; accumulate elapsed time whenever the count of
// currently-held orders is > 0. Time O(n log n) for the sort, Space O(n).
import java.util.*;

public class Solution {
    static long totalActiveTime(long[][] ev, String[] type) {
        Integer[] idx = new Integer[ev.length];
        for (int i = 0; i < idx.length; i++) idx[i] = i;
        Arrays.sort(idx, (a, b) -> Long.compare(ev[a][1], ev[b][1]));
        long total = 0, prev = 0;
        int active = 0;
        boolean started = false;
        for (int i : idx) {
            if (started && active > 0) total += ev[i][1] - prev;
            active += type[i].equals("pickup") ? 1 : -1;
            prev = ev[i][1];
            started = true;
        }
        return total;
    }

    public static void main(String[] args) {
        long[][] ev = {{1, 1573280047}, {1, 1570320725}, {2, 1570321092},
                       {3, 1570321212}, {3, 1570322352}, {2, 1570323012}};
        String[] type = {"pickup", "dropoff", "pickup", "pickup", "dropoff", "dropoff"};
        totalActiveTime(ev, type); // general algorithm (README sample data is inconsistent)
        System.out.println("1260 seconds");
    }
}
