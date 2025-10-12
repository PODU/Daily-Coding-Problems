// Day 419: Min steps to reduce N to 1 (decrement, or jump to larger factor).
// BFS over values 1..N. Time: O(N*sqrt(N)), Space: O(N).
import java.util.ArrayDeque;
import java.util.Queue;

public class Solution {
    static int minSteps(int N) {
        if (N <= 1) return 0;
        int[] dist = new int[N + 1];
        java.util.Arrays.fill(dist, -1);
        Queue<Integer> q = new ArrayDeque<>();
        dist[N] = 0;
        q.add(N);
        while (!q.isEmpty()) {
            int v = q.poll();
            if (v == 1) return dist[1];
            if (v - 1 >= 1 && dist[v - 1] == -1) {
                dist[v - 1] = dist[v] + 1;
                q.add(v - 1);
            }
            for (int a = 2; (long) a * a <= v; a++) {
                if (v % a == 0) {
                    int larger = v / a;
                    if (dist[larger] == -1) {
                        dist[larger] = dist[v] + 1;
                        q.add(larger);
                    }
                }
            }
        }
        return dist[1];
    }

    public static void main(String[] args) {
        int N = 100;
        System.out.println(minSteps(N) + "  (route: 100 -> 10 -> 9 -> 3 -> 2 -> 1)");
    }
}
