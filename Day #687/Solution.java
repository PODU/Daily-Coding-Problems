// 8-puzzle solver via A* with Manhattan-distance heuristic (admissible -> optimal solution).
// State = 9-int board (blank=0); explore by sliding a tile into the blank.
import java.util.*;

public class Solution {
    static int manhattan(int[] b) {
        int d = 0;
        for (int i = 0; i < 9; i++) {
            int v = b[i];
            if (v == 0) continue;
            int gr = (v - 1) / 3, gc = (v - 1) % 3, r = i / 3, c = i % 3;
            d += Math.abs(gr - r) + Math.abs(gc - c);
        }
        return d;
    }

    static String key(int[] b) {
        StringBuilder s = new StringBuilder();
        for (int x : b) s.append(x);
        return s.toString();
    }

    static int[] fromKey(String k) {
        int[] b = new int[9];
        for (int i = 0; i < 9; i++) b[i] = k.charAt(i) - '0';
        return b;
    }

    static class Node { int f; String k; Node(int f, String k) { this.f = f; this.k = k; } }

    public static void main(String[] args) {
        int[] start = {1, 2, 3, 4, 0, 6, 7, 5, 8};
        int[] goal  = {1, 2, 3, 4, 5, 6, 7, 8, 0};
        String startK = key(start), goalK = key(goal);

        Map<String, Integer> g = new HashMap<>();
        Map<String, String> parent = new HashMap<>();
        PriorityQueue<Node> pq = new PriorityQueue<>((a, b) -> a.f - b.f);

        g.put(startK, 0);
        pq.add(new Node(manhattan(start), startK));
        int[] dr = {-1, 1, 0, 0}, dc = {0, 0, -1, 1};

        while (!pq.isEmpty()) {
            Node cur = pq.poll();
            if (cur.k.equals(goalK)) break;
            int cg = g.get(cur.k);
            int[] b = fromKey(cur.k);
            if (cur.f > cg + manhattan(b)) continue; // stale entry
            int z = 0;
            for (int i = 0; i < 9; i++) if (b[i] == 0) { z = i; break; }
            int zr = z / 3, zc = z % 3;
            for (int m = 0; m < 4; m++) {
                int nr = zr + dr[m], nc = zc + dc[m];
                if (nr < 0 || nr >= 3 || nc < 0 || nc >= 3) continue;
                int nz = nr * 3 + nc;
                int[] nb = b.clone();
                int t = nb[z]; nb[z] = nb[nz]; nb[nz] = t;
                String nk = key(nb);
                int ng = cg + 1;
                if (!g.containsKey(nk) || ng < g.get(nk)) {
                    g.put(nk, ng);
                    parent.put(nk, cur.k);
                    pq.add(new Node(ng + manhattan(nb), nk));
                }
            }
        }

        List<String> path = new ArrayList<>();
        String k = goalK;
        path.add(k);
        while (!k.equals(startK)) { k = parent.get(k); path.add(k); }
        Collections.reverse(path);

        int moves = path.size() - 1;
        System.out.println("Solved in " + moves + " moves");
        System.out.println("Goal board:");
        String gb = path.get(path.size() - 1);
        for (int i = 0; i < 9; i++)
            System.out.print(gb.charAt(i) + (i % 3 == 2 ? "\n" : " "));
    }
}
