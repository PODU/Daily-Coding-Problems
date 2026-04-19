// Circular lock min moves via BFS over 1000 states; each state has 6 neighbors (3 wheels x +/-1 mod 10). O(states) time/space.
import java.util.*;

public class Solution {
    static int minMoves(String target, Set<String> deadends) {
        if (deadends.contains("000") || deadends.contains(target)) return -1;
        if (target.equals("000")) return 0;
        Set<String> visited = new HashSet<>();
        visited.add("000");
        Queue<String> q = new LinkedList<>();
        q.add("000");
        int dist = 0;
        while (!q.isEmpty()) {
            int sz = q.size();
            dist++;
            for (int s = 0; s < sz; s++) {
                char[] cur = q.poll().toCharArray();
                for (int i = 0; i < 3; i++) {
                    for (int delta : new int[]{1, 9}) {
                        char[] nx = cur.clone();
                        nx[i] = (char) ('0' + ((cur[i] - '0' + delta) % 10));
                        String s2 = new String(nx);
                        if (visited.contains(s2) || deadends.contains(s2)) continue;
                        if (s2.equals(target)) return dist;
                        visited.add(s2);
                        q.add(s2);
                    }
                }
            }
        }
        return -1;
    }

    public static void main(String[] args) {
        Set<String> deadends = new HashSet<>(Arrays.asList("100", "020", "001"));
        System.out.println(minMoves("333", deadends));
    }
}
