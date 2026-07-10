// Collect positions p_i of people, set q_i = p_i - i, answer = sum |q_i - median(q)|.
// Time O(n), Space O(m).
import java.util.ArrayList;

public class Solution {
    static long minCost(int[] seats) {
        ArrayList<Integer> q = new ArrayList<>();
        int i = 0;
        for (int j = 0; j < seats.length; j++)
            if (seats[j] == 1) q.add(j - i++);
        if (q.isEmpty()) return 0;
        int med = q.get(q.size() / 2);
        long total = 0;
        for (int v : q) total += Math.abs(v - med);
        return total;
    }
    public static void main(String[] args) {
        int[] seats = {0, 1, 1, 0, 1, 0, 0, 0, 1};
        System.out.println(minCost(seats)); // expected 5
    }
}
