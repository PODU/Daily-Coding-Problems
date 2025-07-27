// Game of Life on infinite board: track live cells in a set, count neighbors via a
// map over live cells + neighbors each step. O(live*9) per step. Print cropped board.
import java.util.*;

public class Solution {
    static Set<Long> step(Set<Long> live) {
        Map<Long,Integer> counts = new HashMap<>();
        for (long c : live) {
            int x = (int)(c >> 32), y = (int)(c & 0xffffffffL);
            for (int dx = -1; dx <= 1; dx++)
                for (int dy = -1; dy <= 1; dy++)
                    if (dx != 0 || dy != 0) {
                        long key = (((long)(x+dx)) << 32) | ((y+dy) & 0xffffffffL);
                        counts.merge(key, 1, Integer::sum);
                    }
        }
        Set<Long> next = new HashSet<>();
        for (Map.Entry<Long,Integer> e : counts.entrySet()) {
            int n = e.getValue();
            boolean alive = live.contains(e.getKey());
            if (n == 3 || (alive && n == 2)) next.add(e.getKey());
        }
        return next;
    }

    static long key(int x, int y) { return (((long)x) << 32) | (y & 0xffffffffL); }

    static void printBoard(Set<Long> live) {
        int minx = Integer.MAX_VALUE, maxx = Integer.MIN_VALUE;
        int miny = Integer.MAX_VALUE, maxy = Integer.MIN_VALUE;
        for (long c : live) {
            int x = (int)(c >> 32), y = (int)(c & 0xffffffffL);
            minx = Math.min(minx, x); maxx = Math.max(maxx, x);
            miny = Math.min(miny, y); maxy = Math.max(maxy, y);
        }
        StringBuilder sb = new StringBuilder();
        for (int y = miny; y <= maxy; y++) {
            for (int x = minx; x <= maxx; x++)
                sb.append(live.contains(key(x, y)) ? '*' : '.');
            sb.append("\n");
        }
        System.out.print(sb);
    }

    public static void main(String[] args) {
        Set<Long> live = new HashSet<>(Arrays.asList(key(0,1), key(1,1), key(2,1)));
        int steps = 2;
        for (int s = 0; s <= steps; s++) {
            System.out.println("Step " + s + ":");
            printBoard(live);
            if (s < steps) live = step(live);
        }
    }
}
