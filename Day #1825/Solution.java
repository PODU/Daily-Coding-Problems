// Word circle = Eulerian circuit in graph where each word is an edge first->last char.
// Check in==out degrees, then Hierholzer to build chain. O(V+E).
import java.util.*;

public class Solution {
    static String solve(List<String> words){
        Map<Character, List<String[]>> adj = new HashMap<>();
        Map<Character, Integer> indeg = new HashMap<>(), outdeg = new HashMap<>(), idx = new HashMap<>();
        for(String w : words){
            char a = w.charAt(0), b = w.charAt(w.length()-1);
            adj.computeIfAbsent(a, k -> new ArrayList<>()).add(new String[]{w, String.valueOf(b)});
            outdeg.merge(a, 1, Integer::sum);
            indeg.merge(b, 1, Integer::sum);
        }
        Set<Character> nodes = new HashSet<>();
        nodes.addAll(indeg.keySet()); nodes.addAll(outdeg.keySet());
        for(char c : nodes)
            if(indeg.getOrDefault(c,0).intValue() != outdeg.getOrDefault(c,0).intValue())
                return "No circle";

        char start = words.get(0).charAt(0);
        Deque<Character> st = new ArrayDeque<>();
        st.push(start);
        Deque<String> edgeStack = new ArrayDeque<>();
        List<String> circuit = new ArrayList<>();
        while(!st.isEmpty()){
            char u = st.peek();
            List<String[]> lst = adj.getOrDefault(u, Collections.emptyList());
            int i = idx.getOrDefault(u, 0);
            if(i < lst.size()){
                idx.put(u, i+1);
                String[] pr = lst.get(i);
                st.push(pr[1].charAt(0));
                edgeStack.push(pr[0]);
            } else {
                st.pop();
                if(!edgeStack.isEmpty()) circuit.add(edgeStack.pop());
            }
        }
        if(circuit.size() != words.size()) return "No circle";
        Collections.reverse(circuit);
        StringBuilder sb = new StringBuilder(circuit.get(0));
        for(int i = 1; i < circuit.size(); i++) sb.append(" --> ").append(circuit.get(i));
        sb.append(" --> ").append(circuit.get(0));
        return sb.toString();
    }
    public static void main(String[] args){
        List<String> words = Arrays.asList("chair","height","racket","touch","tunic");
        System.out.println(solve(words));
    }
}
