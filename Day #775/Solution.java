// Day 775: Minimum meeting rooms = max overlapping intervals.
// Sort starts and ends, two-pointer sweep. O(n log n) time, O(n) space.
import java.util.*;

public class Solution {
    static int minRooms(int[][] iv) {
        int n = iv.length;
        int[] starts = new int[n], ends = new int[n];
        for (int i = 0; i < n; i++) { starts[i] = iv[i][0]; ends[i] = iv[i][1]; }
        Arrays.sort(starts);
        Arrays.sort(ends);
        int rooms = 0, best = 0, i = 0, j = 0;
        while (i < n) {
            if (starts[i] < ends[j]) { rooms++; i++; best = Math.max(best, rooms); }
            else { rooms--; j++; }
        }
        return best;
    }

    public static void main(String[] args) {
        System.out.println(minRooms(new int[][]{{30, 75}, {0, 50}, {60, 150}})); // 2
    }
}
