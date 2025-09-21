// Day 309: Min movement to pack people with no gaps. Map p_i - i; cost minimized
// at the median of those values. O(M log M).
import java.util.*;
public class Solution {
    static long minCost(int[] seats) {
        List<Long> b = new ArrayList<>();
        int idx = 0;
        for (int i = 0; i < seats.length; i++) if (seats[i] == 1) b.add((long) (i - (idx++)));
        if (b.isEmpty()) return 0;
        Collections.sort(b);
        long med = b.get(b.size() / 2), cost = 0;
        for (long x : b) cost += Math.abs(x - med);
        return cost;
    }
    public static void main(String[] a) {
        int[] s = {0, 1, 1, 0, 1, 0, 0, 0, 1};
        System.out.println(minCost(s)); // 5
    }
}
