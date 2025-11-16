// Day 607: Min total movement to seat M people contiguously in a row.
// Approach: target = median of (pos[i]-i); cost = sum |(pos[i]-i) - median|. Time O(n), Space O(M).
import java.util.*;

public class Solution {
    static long minCost(int[] seats) {
        List<Long> b = new ArrayList<>();
        int idx = 0;
        for (int i = 0; i < seats.length; i++)
            if (seats[i] == 1) b.add((long) (i - idx++));
        if (b.isEmpty()) return 0;
        Collections.sort(b);
        long med = b.get(b.size() / 2);
        long cost = 0;
        for (long v : b) cost += Math.abs(v - med);
        return cost;
    }

    public static void main(String[] args) {
        int[] seats = {0, 1, 1, 0, 1, 0, 0, 0, 1};
        System.out.println(minCost(seats)); // 5
    }
}
