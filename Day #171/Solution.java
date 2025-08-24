// Busiest period: sort events by timestamp, sweep occupancy, track max-occupancy interval. O(n log n) time, O(n) space.
import java.util.*;

public class Solution {
    enum Type { ENTER, EXIT }
    static class Event {
        long ts; int count; Type type;
        Event(long ts, int count, Type type) { this.ts = ts; this.count = count; this.type = type; }
    }

    static long[] busiestPeriod(List<Event> events) {
        events.sort(Comparator.comparingLong(e -> e.ts));
        long occ = 0, maxOcc = -1, bestStart = 0, bestEnd = 0;
        for (int i = 0; i < events.size(); i++) {
            Event e = events.get(i);
            occ += (e.type == Type.ENTER) ? e.count : -e.count;
            if (occ > maxOcc && i + 1 < events.size()) {
                maxOcc = occ;
                bestStart = e.ts;
                bestEnd = events.get(i + 1).ts;
            }
        }
        return new long[]{bestStart, bestEnd};
    }

    public static void main(String[] args) {
        List<Event> events = new ArrayList<>(Arrays.asList(
            new Event(1526579928L, 3, Type.ENTER),
            new Event(1526580382L, 2, Type.EXIT),
            new Event(1526579999L, 1, Type.ENTER),
            new Event(1526580001L, 5, Type.ENTER)
        ));
        long[] r = busiestPeriod(events);
        System.out.println("(" + r[0] + ", " + r[1] + ")");
    }
}
