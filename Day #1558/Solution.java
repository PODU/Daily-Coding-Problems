// Count connected components (friend groups) in an undirected graph via DFS.
// Time O(V+E), Space O(V).
import java.util.*;

public class Solution {
    public static void main(String[] args) {
        Map<Integer, List<Integer>> adj = new LinkedHashMap<>();
        adj.put(0, Arrays.asList(1, 2));
        adj.put(1, Arrays.asList(0, 5));
        adj.put(2, Arrays.asList(0));
        adj.put(3, Arrays.asList(6));
        adj.put(4, Arrays.asList());
        adj.put(5, Arrays.asList(1));
        adj.put(6, Arrays.asList(3));

        Set<Integer> visited = new HashSet<>();
        int groups = 0;
        for (int start : adj.keySet()) {
            if (visited.contains(start)) continue;
            groups++;
            Deque<Integer> stack = new ArrayDeque<>();
            stack.push(start);
            visited.add(start);
            while (!stack.isEmpty()) {
                int u = stack.pop();
                for (int v : adj.get(u)) {
                    if (!visited.contains(v)) {
                        visited.add(v);
                        stack.push(v);
                    }
                }
            }
        }
        System.out.println(groups);
    }
}
