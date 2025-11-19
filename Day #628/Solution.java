// Combination lock: BFS over 1000 states from "000" to target, avoiding deadends.
// Each of 3 wheels has 2 neighbors (+1/-1 mod 10) => 6 neighbors. Time/space O(1000).
import java.util.*;

public class Solution {
    static int openLock(String[] deadends, String target, String start) {
        Set<String> dead = new HashSet<>(Arrays.asList(deadends));
        if (dead.contains(start)) return -1;
        if (start.equals(target)) return 0;
        Set<String> visited = new HashSet<>();
        visited.add(start);
        Queue<String> q = new ArrayDeque<>();
        q.add(start);
        int steps = 0;
        while (!q.isEmpty()) {
            int sz = q.size();
            steps++;
            for (int i = 0; i < sz; i++) {
                String cur = q.poll();
                for (int w = 0; w < 3; w++) {
                    for (int d = -1; d <= 1; d += 2) {
                        char[] arr = cur.toCharArray();
                        arr[w] = (char) ('0' + ((arr[w] - '0' + d + 10) % 10));
                        String nxt = new String(arr);
                        if (dead.contains(nxt) || visited.contains(nxt)) continue;
                        if (nxt.equals(target)) return steps;
                        visited.add(nxt);
                        q.add(nxt);
                    }
                }
            }
        }
        return -1;
    }

    public static void main(String[] args) {
        String[] dead1 = {"010", "021"};
        int r1 = openLock(dead1, "020", "000");
        System.out.println("Min moves to open lock (target 020): " + r1);

        String[] dead2 = {"001","010","100","009","090","900"};
        int r2 = openLock(dead2, "111", "000");
        System.out.println("Impossible case (target 111): " + (r2 == -1 ? "None" : r2));
    }
}
