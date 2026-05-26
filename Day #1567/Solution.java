// Busiest period: sort events by timestamp, sweep current occupancy, track max interval.
// Time O(n log n), space O(n).
import java.util.*;

public class Solution {
    static class Event {
        long ts, count; boolean enter;
        Event(long ts, long count, boolean enter) { this.ts = ts; this.count = count; this.enter = enter; }
    }

    static long[] busiestPeriod(List<Event> events) {
        events.sort(Comparator.comparingLong(e -> e.ts));
        long cur = 0, best = -1, bestStart = 0, bestEnd = 0;
        for (int i = 0; i < events.size(); i++) {
            Event e = events.get(i);
            cur += e.enter ? e.count : -e.count;
            if (cur > best && i + 1 < events.size()) {
                best = cur;
                bestStart = e.ts;
                bestEnd = events.get(i + 1).ts;
            }
        }
        return new long[]{bestStart, bestEnd};
    }

    public static void main(String[] args) {
        List<Event> events = new ArrayList<>();
        events.add(new Event(1, 3, true));
        events.add(new Event(4, 2, true));
        events.add(new Event(6, 5, false));
        long[] r = busiestPeriod(events);
        System.out.println("(" + r[0] + ", " + r[1] + ")");
    }
}
