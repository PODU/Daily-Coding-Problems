// BFS over squares 1..100; from s try rolls 1..6, apply snake/ladder mapping. Time O(N*6), Space O(N).
import java.util.*;

public class Solution {
    static int minTurns() {
        Map<Integer, Integer> jump = new HashMap<>();
        int[][] snakes = {{16,6},{48,26},{49,11},{56,53},{62,19},{64,60},{87,24},{93,73},{95,75},{98,78}};
        int[][] ladders = {{1,38},{4,14},{9,31},{21,42},{28,84},{36,44},{51,67},{71,91},{80,100}};
        for (int[] s : snakes) jump.put(s[0], s[1]);
        for (int[] l : ladders) jump.put(l[0], l[1]);

        int[] dist = new int[101];
        Arrays.fill(dist, -1);
        Queue<Integer> q = new ArrayDeque<>();
        q.add(1); dist[1] = 0;                  // start placed on 1; do NOT apply 1->38 here
        while (!q.isEmpty()) {
            int s = q.poll();
            if (s == 100) return dist[s];
            for (int r = 1; r <= 6; r++) {
                int nxt = s + r;
                if (nxt > 100) continue;
                if (jump.containsKey(nxt)) nxt = jump.get(nxt);
                if (dist[nxt] == -1) { dist[nxt] = dist[s] + 1; q.add(nxt); }
            }
        }
        return dist[100];
    }

    public static void main(String[] args) {
        System.out.println("Minimum turns: " + minTurns());
    }
}
