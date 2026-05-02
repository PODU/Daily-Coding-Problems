// Snakes & Ladders: BFS over squares 1..100, dice rolls 1..6, apply jumps. Min turns 1->100.
// Time: O(100*6). Space: O(100).
import java.util.ArrayDeque;
import java.util.HashMap;
import java.util.Map;
import java.util.Queue;

public class Solution {

    static Map<Integer, Integer> jump = new HashMap<>();

    static int land(int s) {
        return jump.getOrDefault(s, s);
    }

    static int minTurns() {
        int[][] snakes = {{16,6},{48,26},{49,11},{56,53},{62,19},{64,60},{87,24},{93,73},{95,75},{98,78}};
        int[][] ladders = {{1,38},{4,14},{9,31},{21,42},{28,84},{36,44},{51,67},{71,91},{80,100}};
        for (int[] s : snakes) jump.put(s[0], s[1]);
        for (int[] l : ladders) jump.put(l[0], l[1]);

        int start = land(1);
        int[] dist = new int[101];
        java.util.Arrays.fill(dist, -1);
        Queue<Integer> q = new ArrayDeque<>();
        dist[start] = 0;
        q.add(start);
        while (!q.isEmpty()) {
            int s = q.poll();
            if (s == 100) return dist[s];
            for (int d = 1; d <= 6; d++) {
                int nxt = s + d;
                if (nxt > 100) continue;
                nxt = land(nxt);
                if (dist[nxt] == -1) {
                    dist[nxt] = dist[s] + 1;
                    q.add(nxt);
                }
            }
        }
        return -1;
    }

    public static void main(String[] args) {
        System.out.println(minTurns());
    }
}
