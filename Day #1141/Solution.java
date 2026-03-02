// Day 1141: Min cost to pack people (remove gaps).
// Transform p_i -> p_i - i, answer = sum |q_i - median(q)|. Time O(n log n), Space O(m).
import java.util.*;

public class Solution {
    static long minCost(int[] seats) {
        List<Long> q = new ArrayList<>();
        int idx = 0;
        for (int i = 0; i < seats.length; i++)
            if (seats[i] == 1) q.add((long) (i - idx++));
        if (q.isEmpty()) return 0;
        Collections.sort(q);
        long med = q.get(q.size() / 2), cost = 0;
        for (long v : q) cost += Math.abs(v - med);
        return cost;
    }

    public static void main(String[] args) {
        int[] seats = {0, 1, 1, 0, 1, 0, 0, 0, 1};
        System.out.println(minCost(seats)); // 5
    }
}
