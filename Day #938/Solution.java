// Day 938: Min moves on a 3-wheel lock from 000 to target, avoiding dead ends. BFS over
// 1000 states, each with 6 neighbors. Time O(1000*6), Space O(1000). Returns -1 (None) if blocked.
import java.util.*;

public class Solution {
    static int minMoves(String target, Set<String> dead) {
        String start = "000";
        if (dead.contains(start) || dead.contains(target)) return -1;
        if (start.equals(target)) return 0;
        Set<String> visited = new HashSet<>();
        visited.add(start);
        Queue<String> q = new ArrayDeque<>();
        q.add(start);
        int depth = 0;
        while (!q.isEmpty()) {
            int sz = q.size();
            depth++;
            for (int s = 0; s < sz; s++) {
                char[] cur = q.poll().toCharArray();
                for (int i = 0; i < 3; i++) {
                    for (int dir : new int[]{1, 9}) {
                        char[] nx = cur.clone();
                        nx[i] = (char) ('0' + ((cur[i] - '0') + dir) % 10);
                        String ns = new String(nx);
                        if (dead.contains(ns) || visited.contains(ns)) continue;
                        if (ns.equals(target)) return depth;
                        visited.add(ns);
                        q.add(ns);
                    }
                }
            }
        }
        return -1;
    }

    public static void main(String[] args) {
        System.out.println(minMoves("123", new HashSet<>())); // 6
        Set<String> dead = new HashSet<>(Arrays.asList(
                "100", "900", "010", "090", "001", "009"));
        int r = minMoves("111", dead);
        System.out.println(r == -1 ? "None" : Integer.toString(r)); // None
    }
}
