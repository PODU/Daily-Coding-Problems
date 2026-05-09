// Day 1498: Conway's Game of Life on an infinite board using a hash set of
// live "row,col" cells; per step tally neighbor counts over live cells.
// Time O(L) per step (L = live cells), Space O(L).
import java.util.*;

public class Solution {
    static class GameOfLife {
        Set<Long> live = new HashSet<>();
        static long key(int r, int c) { return (((long) r) << 32) ^ (c & 0xffffffffL); }
        static int row(long k) { return (int) (k >> 32); }
        static int col(long k) { return (int) k; }

        GameOfLife(int[][] cells) {
            for (int[] cell : cells) live.add(key(cell[0], cell[1]));
        }
        void step() {
            Map<Long, Integer> counts = new HashMap<>();
            for (long k : live) {
                int r = row(k), c = col(k);
                for (int dr = -1; dr <= 1; dr++)
                    for (int dc = -1; dc <= 1; dc++)
                        if (dr != 0 || dc != 0)
                            counts.merge(key(r + dr, c + dc), 1, Integer::sum);
            }
            Set<Long> next = new HashSet<>();
            for (Map.Entry<Long, Integer> e : counts.entrySet()) {
                int n = e.getValue();
                boolean alive = live.contains(e.getKey());
                if (n == 3 || (alive && n == 2)) next.add(e.getKey());
            }
            live = next;
        }
        void print() {
            if (live.isEmpty()) { System.out.println("(empty)"); return; }
            int minR = Integer.MAX_VALUE, maxR = Integer.MIN_VALUE;
            int minC = Integer.MAX_VALUE, maxC = Integer.MIN_VALUE;
            for (long k : live) {
                minR = Math.min(minR, row(k)); maxR = Math.max(maxR, row(k));
                minC = Math.min(minC, col(k)); maxC = Math.max(maxC, col(k));
            }
            for (int r = minR; r <= maxR; r++) {
                StringBuilder sb = new StringBuilder();
                for (int c = minC; c <= maxC; c++)
                    sb.append(live.contains(key(r, c)) ? '*' : '.');
                System.out.println(sb);
            }
        }
    }

    public static void main(String[] args) {
        GameOfLife game = new GameOfLife(new int[][]{{0,1},{1,1},{2,1}});
        int steps = 2;
        System.out.println("Step 0:"); game.print();
        for (int s = 1; s <= steps; s++) {
            game.step();
            System.out.println("Step " + s + ":");
            game.print();
        }
    }
}
