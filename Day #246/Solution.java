// Words form a circle: model each word as a directed edge first->last char; an Eulerian circuit
// exists iff in-degree==out-degree for every node and edges form one connected component.
// Find the circuit with Hierholzer's algorithm. O(V + E) time and space.
import java.util.*;

public class Solution {
    static List<String> circleOrder(List<String> words) {
        Map<Character, List<String[]>> adj = new HashMap<>(); // first -> [lastChar, word]
        Map<Character, Integer> indeg = new HashMap<>(), outdeg = new HashMap<>();
        Set<Character> nodes = new TreeSet<>();
        for (String w : words) {
            char a = w.charAt(0), b = w.charAt(w.length() - 1);
            adj.computeIfAbsent(a, k -> new ArrayList<>()).add(new String[]{String.valueOf(b), w});
            outdeg.merge(a, 1, Integer::sum);
            indeg.merge(b, 1, Integer::sum);
            nodes.add(a); nodes.add(b);
        }
        for (char c : nodes)
            if (!indeg.getOrDefault(c, 0).equals(outdeg.getOrDefault(c, 0))) return null;

        // Connectivity (undirected) over nodes that have outgoing edges.
        Map<Character, List<Character>> und = new HashMap<>();
        for (var e : adj.entrySet())
            for (String[] p : e.getValue()) {
                char a = e.getKey(), b = p[0].charAt(0);
                und.computeIfAbsent(a, k -> new ArrayList<>()).add(b);
                und.computeIfAbsent(b, k -> new ArrayList<>()).add(a);
            }
        Set<Character> active = new TreeSet<>();
        for (var e : adj.entrySet()) if (!e.getValue().isEmpty()) active.add(e.getKey());
        if (active.isEmpty()) return null;
        Set<Character> seen = new HashSet<>();
        Deque<Character> st = new ArrayDeque<>();
        st.push(active.iterator().next());
        while (!st.isEmpty()) {
            char c = st.pop();
            if (!seen.add(c)) continue;
            for (char nb : und.getOrDefault(c, List.of())) if (!seen.contains(nb)) st.push(nb);
        }
        for (char c : active) if (!seen.contains(c)) return null;

        // Hierholzer starting from first word's first char.
        char start = words.get(0).charAt(0);
        Map<Character, Integer> ptr = new HashMap<>();
        Deque<Character> nodeStack = new ArrayDeque<>();
        Deque<String> edgeStack = new ArrayDeque<>();
        List<String> circuit = new ArrayList<>();
        nodeStack.push(start);
        while (!nodeStack.isEmpty()) {
            char v = nodeStack.peek();
            List<String[]> lst = adj.getOrDefault(v, List.of());
            int p = ptr.getOrDefault(v, 0);
            if (p < lst.size()) {
                ptr.put(v, p + 1);
                String[] edge = lst.get(p);
                nodeStack.push(edge[0].charAt(0));
                edgeStack.push(edge[1]);
            } else {
                nodeStack.pop();
                if (!edgeStack.isEmpty()) circuit.add(edgeStack.pop());
            }
        }
        if (circuit.size() != words.size()) return null;
        Collections.reverse(circuit);
        return circuit;
    }

    public static void main(String[] args) {
        List<String> words = Arrays.asList("chair", "height", "racket", "touch", "tunic");
        List<String> order = circleOrder(words);
        if (order != null) {
            StringBuilder sb = new StringBuilder();
            for (String w : order) sb.append(w).append(" --> ");
            sb.append(order.get(0));
            System.out.println(sb);
        } else {
            System.out.println("No circle possible");
        }
    }
}
