// Busiest period: sort events by timestamp, sweep occupancy, track max-occupancy interval [t_i, t_{i+1}). O(n log n) time, O(n) space.
import java.util.Arrays;
import java.util.Comparator;

public class Solution {
    static class Event {
        long timestamp; int count; String type;
        Event(long t, int c, String ty) { timestamp = t; count = c; type = ty; }
    }

    public static void main(String[] args) {
        Event[] events = {
            new Event(1526579928, 3, "enter"),
            new Event(1526580000, 2, "enter"),
            new Event(1526580382, 2, "exit"),
            new Event(1526580500, 1, "enter"),
            new Event(1526580700, 4, "exit")
        };
        Arrays.sort(events, Comparator.comparingLong(e -> e.timestamp));

        long current = 0, maxOcc = -1, bestStart = 0, bestEnd = 0;
        for (int i = 0; i < events.length; i++) {
            if (events[i].type.equals("enter")) current += events[i].count;
            else current -= events[i].count;
            long nextTs = (i + 1 < events.length) ? events[i + 1].timestamp : events[i].timestamp;
            if (current > maxOcc) {
                maxOcc = current;
                bestStart = events[i].timestamp;
                bestEnd = nextTs;
            }
        }
        System.out.println("(" + bestStart + ", " + bestEnd + ")");
    }
}
