// Meeting rooms: sort starts & ends, sweep with two pointers tracking overlap.
// Time O(n log n), Space O(n).
import java.util.Arrays;

public class Solution {
    static int minRooms(int[][] intervals) {
        int n = intervals.length;
        int[] starts = new int[n];
        int[] ends = new int[n];
        for (int i = 0; i < n; i++) {
            starts[i] = intervals[i][0];
            ends[i] = intervals[i][1];
        }
        Arrays.sort(starts);
        Arrays.sort(ends);
        int rooms = 0, maxRooms = 0, j = 0;
        for (int i = 0; i < n; i++) {
            while (j < n && ends[j] <= starts[i]) { rooms--; j++; }
            rooms++;
            maxRooms = Math.max(maxRooms, rooms);
        }
        return maxRooms;
    }

    public static void main(String[] args) {
        int[][] intervals = {{30, 75}, {0, 50}, {60, 150}};
        System.out.println(minRooms(intervals));
    }
}
