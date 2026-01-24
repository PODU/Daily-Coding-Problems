// Day 950: busiest period - interval with the most people inside the building.
// Sort events by timestamp, sweep maintaining running count. Time O(n log n), Space O(n).
import java.util.*;

public class Solution {
    static class Event {
        long ts; int count; boolean enter;
        Event(long ts, int count, boolean enter) { this.ts = ts; this.count = count; this.enter = enter; }
    }

    static long[] busiest(List<Event> ev) {
        ev.sort(Comparator.comparingLong(e -> e.ts));
        long cur = 0, best = Long.MIN_VALUE;
        long[] ans = new long[2];
        for (int i = 0; i < ev.size(); i++) {
            cur += ev.get(i).enter ? ev.get(i).count : -ev.get(i).count;
            long nextTs = (i + 1 < ev.size()) ? ev.get(i + 1).ts : ev.get(i).ts;
            if (cur > best) { best = cur; ans[0] = ev.get(i).ts; ans[1] = nextTs; }
        }
        return ans;
    }

    public static void main(String[] args) {
        List<Event> ev = new ArrayList<>(Arrays.asList(
            new Event(1526579928, 3, true),
            new Event(1526579943, 4, true),
            new Event(1526580382, 2, false),
            new Event(1526581000, 5, false)
        ));
        long[] p = busiest(ev);
        System.out.println("(" + p[0] + ", " + p[1] + ")"); // (1526579943, 1526580382)
    }
}
