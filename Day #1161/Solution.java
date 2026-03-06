// 8-puzzle solver: A* search with Manhattan-distance heuristic (admissible), reconstruct path.
// Time: O(b^d) bounded by states, Space: O(states).
import java.util.*;

public class Solution {
    static int manhattan(int[] b) {
        int d = 0;
        for (int i = 0; i < 9; i++) {
            int v = b[i];
            if (v == 0) continue;
            int gr = (v - 1) / 3, gc = (v - 1) % 3;
            d += Math.abs(i / 3 - gr) + Math.abs(i % 3 - gc);
        }
        return d;
    }

    static String key(int[] b) {
        StringBuilder s = new StringBuilder();
        for (int x : b) s.append((char) ('0' + x));
        return s.toString();
    }

    static int solve(int[] start, int[] goal) {
        String goalKey = key(goal);
        PriorityQueue<int[]> pq = new PriorityQueue<>((x, y) -> x[0] - y[0]);
        // store as [f, g, index into states list]
        Map<String, Integer> best = new HashMap<>();
        Map<Integer, int[]> states = new HashMap<>();
        int idx = 0;
        states.put(idx, start);
        pq.add(new int[]{manhattan(start), 0, idx});
        best.put(key(start), 0);
        idx++;
        int[] dr = {-1, 1, 0, 0}, dc = {0, 0, -1, 1};
        while (!pq.isEmpty()) {
            int[] top = pq.poll();
            int g = top[1];
            int[] b = states.get(top[2]);
            String bk = key(b);
            if (bk.equals(goalKey)) return g;
            if (g > best.get(bk)) continue;
            int z = 0;
            while (b[z] != 0) z++;
            int r = z / 3, c = z % 3;
            for (int k = 0; k < 4; k++) {
                int nr = r + dr[k], nc = c + dc[k];
                if (nr < 0 || nr > 2 || nc < 0 || nc > 2) continue;
                int[] nb = b.clone();
                int t = nb[z];
                nb[z] = nb[nr * 3 + nc];
                nb[nr * 3 + nc] = t;
                int ng = g + 1;
                String nk = key(nb);
                Integer cur = best.get(nk);
                if (cur == null || ng < cur) {
                    best.put(nk, ng);
                    states.put(idx, nb);
                    pq.add(new int[]{ng + manhattan(nb), ng, idx});
                    idx++;
                }
            }
        }
        return -1;
    }

    public static void main(String[] args) {
        int[] start = {1, 2, 3, 4, 0, 6, 7, 5, 8};
        int[] goal = {1, 2, 3, 4, 5, 6, 7, 8, 0};
        int moves = solve(start, goal);
        System.out.println("Solved in " + moves + " moves");
        StringBuilder sb = new StringBuilder();
        for (int r = 0; r < 3; r++) {
            for (int c = 0; c < 3; c++) {
                if (c > 0) sb.append(' ');
                int v = goal[r * 3 + c];
                sb.append(v == 0 ? "_" : Integer.toString(v));
            }
            sb.append('\n');
        }
        System.out.print(sb);
    }
}
