// Topological sort of courses (Kahn's algorithm with cycle detection).
// Lexicographic tie-break via min-heap for deterministic order.
// Time: O((V+E) log V), Space: O(V+E).
import java.util.*;

public class Solution {
    // prereqs: course -> list of prerequisite courses.
    static List<String> courseOrder(Map<String, List<String>> prereqs){
        Map<String, List<String>> adj = new HashMap<>();
        Map<String, Integer> indeg = new HashMap<>();
        for(String c : prereqs.keySet()){ indeg.putIfAbsent(c, 0); adj.putIfAbsent(c, new ArrayList<>()); }
        for(Map.Entry<String, List<String>> e : prereqs.entrySet()){
            for(String p : e.getValue()){
                adj.computeIfAbsent(p, k -> new ArrayList<>()).add(e.getKey());
                indeg.merge(e.getKey(), 1, Integer::sum);
                indeg.putIfAbsent(p, 0);
            }
        }
        PriorityQueue<String> pq = new PriorityQueue<>();
        for(Map.Entry<String, Integer> e : indeg.entrySet()) if(e.getValue() == 0) pq.add(e.getKey());
        List<String> order = new ArrayList<>();
        while(!pq.isEmpty()){
            String c = pq.poll();
            order.add(c);
            for(String nx : adj.getOrDefault(c, Collections.emptyList())){
                indeg.merge(nx, -1, Integer::sum);
                if(indeg.get(nx) == 0) pq.add(nx);
            }
        }
        return order.size() == indeg.size() ? order : null;
    }

    public static void main(String[] args){
        Map<String, List<String>> prereqs = new HashMap<>();
        prereqs.put("CSC300", Arrays.asList("CSC100", "CSC200"));
        prereqs.put("CSC200", Arrays.asList("CSC100"));
        prereqs.put("CSC100", new ArrayList<>());
        List<String> order = courseOrder(prereqs);
        if(order == null){ System.out.println("null"); return; }
        StringBuilder sb = new StringBuilder("[");
        for(int i=0;i<order.size();i++){ sb.append("'").append(order.get(i)).append("'"); if(i+1<order.size()) sb.append(", "); }
        sb.append("]");
        System.out.println(sb); // ['CSC100', 'CSC200', 'CSC300']
    }
}
