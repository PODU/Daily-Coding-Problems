// Day 456: Busiest period in a building from enter/exit events.
// Sort by timestamp, sweep occupancy, track interval of max. Time O(n log n).
import java.util.Arrays;
import java.util.Comparator;

public class Solution {
    static class Event {
        long ts; int count; String type;
        Event(long ts, int count, String type) { this.ts = ts; this.count = count; this.type = type; }
    }

    static long[] busiest(Event[] ev) {
        Arrays.sort(ev, Comparator.comparingLong(e -> e.ts));
        long cur = 0, best = -1, bestStart = 0, bestEnd = 0;
        int n = ev.length;
        for (int i = 0; i < n; i++) {
            cur += ev[i].type.equals("enter") ? ev[i].count : -ev[i].count;
            long end = (i + 1 < n) ? ev[i + 1].ts : ev[i].ts;
            if (cur > best) { best = cur; bestStart = ev[i].ts; bestEnd = end; }
        }
        return new long[]{bestStart, bestEnd};
    }

    public static void main(String[] args) {
        Event[] ev = {
            new Event(1526579928L, 3, "enter"),
            new Event(1526579940L, 2, "enter"),
            new Event(1526580000L, 1, "exit"),
            new Event(1526580382L, 4, "exit"),
        };
        long[] r = busiest(ev);
        System.out.println("(" + r[0] + ", " + r[1] + ")");
    }
}
