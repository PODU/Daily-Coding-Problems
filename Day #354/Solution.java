// Distributed crawler design (simulation): Coordinator holds a FIFO URL frontier + central visited set.
// Workers pull a URL, "download" (map lookup), record it, push unvisited links. BFS over a mock graph.
// Architecture: reach pages = BFS from seeds; visited = central set/bloom filter on coordinator;
// blacklisting = rotate IPs/user-agents, rate-limit + backoff; updates = recrawl by last-modified
// priority queue. Time O(V+E), Space O(V).
import java.util.*;

public class Solution {
    public static void main(String[] args) {
        Map<String, List<String>> wiki = new HashMap<>();
        wiki.put("/Main",   Arrays.asList("/Apple", "/Banana"));
        wiki.put("/Apple",  Arrays.asList("/Banana", "/Fruit"));
        wiki.put("/Banana", Arrays.asList("/Fruit"));
        wiki.put("/Fruit",  Arrays.asList("/Main"));

        List<String> seeds = Arrays.asList("/Main");
        Queue<String> frontier = new LinkedList<>();
        Set<String> visited = new HashSet<>();
        List<String> order = new ArrayList<>();

        for (String s : seeds) {
            if (visited.add(s)) frontier.add(s);
        }

        while (!frontier.isEmpty()) {
            String url = frontier.poll();
            order.add(url); // "download" + record into results DB
            for (String link : wiki.getOrDefault(url, Collections.emptyList())) {
                if (visited.add(link)) frontier.add(link);
            }
        }

        System.out.println("Crawled " + order.size() + " pages:");
        for (String p : order) System.out.println(p);
    }
}
