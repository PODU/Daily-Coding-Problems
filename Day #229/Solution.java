// Snakes and Ladders: BFS over squares 1..100, each die roll (1..6) is one edge; apply jumps.
// Time: O(100 * 6), Space: O(100).
import java.util.*;

public class Solution {
    static int minTurns() {
        Map<Integer, Integer> jump = new HashMap<>();
        int[][] pairs = {{16, 6}, {48, 26}, {49, 11}, {56, 53}, {62, 19}, {64, 60}, {87, 24}, {93, 73},
            {95, 75}, {98, 78}, {1, 38}, {4, 14}, {9, 31}, {21, 42}, {28, 84}, {36, 44}, {51, 67}, {71, 91}, {80, 100}};
        for (int[] p : pairs) jump.put(p[0], p[1]);
        int[] dist = new int[101];
        Arrays.fill(dist, -1);
        int start = jump.getOrDefault(1, 1);
        Queue<Integer> q = new LinkedList<>();
        q.add(start);
        dist[start] = 0;
        while (!q.isEmpty()) {
            int p = q.poll();
            if (p == 100) return dist[p];
            for (int d = 1; d <= 6; d++) {
                int np = p + d;
                if (np > 100) continue;
                np = jump.getOrDefault(np, np);
                if (dist[np] == -1) { dist[np] = dist[p] + 1; q.add(np); }
            }
        }
        return dist[100];
    }

    public static void main(String[] args) {
        System.out.println("Minimum turns: " + minTurns());
    }
}
