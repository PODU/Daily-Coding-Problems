// Snakes & Ladders minimum turns: BFS over board squares (unweighted shortest path).
// Each square has up to 6 die-roll edges; snakes/ladders redirect the landing square.
// Time: O(100*6). Space: O(100).
import java.util.*;

public class Solution {
    static int minTurns(Map<Integer, Integer> jump, int size) {
        int[] dist = new int[size + 1];
        Arrays.fill(dist, -1);
        Queue<Integer> q = new LinkedList<>();
        dist[1] = 0; q.add(1); // begin on square 1; jumps trigger only on rolled squares
        while (!q.isEmpty()) {
            int sq = q.poll();
            if (sq == size) return dist[sq];
            for (int d = 1; d <= 6; d++) {
                int nxt = sq + d;
                if (nxt > size) continue;
                if (jump.containsKey(nxt)) nxt = jump.get(nxt);
                if (dist[nxt] == -1) { dist[nxt] = dist[sq] + 1; q.add(nxt); }
            }
        }
        return dist[size];
    }

    public static void main(String[] args) {
        Map<Integer, Integer> jump = new HashMap<>();
        int[][] sl = {{16,6},{48,26},{49,11},{56,53},{62,19},{64,60},{87,24},{93,73},{95,75},{98,78},
                      {1,38},{4,14},{9,31},{21,42},{28,84},{36,44},{51,67},{71,91},{80,100}};
        for (int[] e : sl) jump.put(e[0], e[1]);
        System.out.println(minTurns(jump, 100)); // 7
    }
}
