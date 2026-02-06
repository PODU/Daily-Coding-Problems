// Day 1031: Snakes & Ladders min turns. BFS over squares 1..100, each turn rolls 1..6,
// applying snake/ladder mapping when a roll lands on a key. Time O(100*6), Space O(100).
import java.util.*;

public class Solution {
    static int minTurns() {
        Map<Integer, Integer> jumps = new HashMap<>();
        int[][] m = {{16, 6}, {48, 26}, {49, 11}, {56, 53}, {62, 19}, {64, 60}, {87, 24}, {93, 73},
                {95, 75}, {98, 78}, {1, 38}, {4, 14}, {9, 31}, {21, 42}, {28, 84}, {36, 44},
                {51, 67}, {71, 91}, {80, 100}};
        for (int[] e : m) jumps.put(e[0], e[1]);
        int[] dist = new int[101];
        Arrays.fill(dist, -1);
        Deque<Integer> q = new ArrayDeque<>();
        dist[1] = 0;
        q.add(1);
        while (!q.isEmpty()) {
            int s = q.poll();
            if (s == 100) return dist[s];
            for (int d = 1; d <= 6; d++) {
                int nx = s + d;
                if (nx > 100) continue;
                if (jumps.containsKey(nx)) nx = jumps.get(nx);
                if (dist[nx] == -1) { dist[nx] = dist[s] + 1; q.add(nx); }
            }
        }
        return -1;
    }

    public static void main(String[] args) {
        System.out.println(minTurns());
    }
}
