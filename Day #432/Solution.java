// Day 432: Distributed Wikipedia crawler — concrete single-process simulation.
// Design: a central coordinator holds a URL frontier (BFS queue) + visited set for dedup.
// Workers pull URLs, "fetch" pages from an in-memory graph, parse links, store content keyed
// by a content hash + last-crawled timestamp, and push new URLs back. Politeness delay per
// domain + worker blacklist/rotation prevent overload. Incremental re-crawl compares the
// stored content hash to detect updates. O(V+E) over the page graph.
import java.util.*;
import java.nio.charset.StandardCharsets;

public class Solution {
    static long fnv1a(String s) {
        long h = 2166136261L;
        for (byte b : s.getBytes(StandardCharsets.UTF_8)) {
            h ^= (b & 0xFF);
            h = (h * 16777619L) & 0xFFFFFFFFL;
        }
        return h;
    }

    static Map<String, Object[]> wiki = new HashMap<>(); // url -> {String content, String[] links}
    static Deque<String> frontier = new ArrayDeque<>();
    static Set<String> visited = new HashSet<>();
    static Map<String, long[]> db = new HashMap<>();      // url -> {hash, ts}
    static List<String> order = new ArrayList<>();
    static String[] workers = {"w0", "w1", "w2"};
    static Map<String, Integer> reqCount = new HashMap<>();
    static Set<String> blacklisted = new HashSet<>();
    static long lastAccess = 0;
    static int wi = 0;

    static String pickWorker() {
        for (int k = 0; k < workers.length; k++) {
            String w = workers[wi % workers.length];
            wi++;
            if (!blacklisted.contains(w)) return w;
        }
        blacklisted.clear();
        return workers[0];
    }
    static Object[] fetch(String worker, String url) {
        lastAccess++;                                       // politeness tick
        int rc = reqCount.merge(worker, 1, Integer::sum);
        if (rc >= 2) blacklisted.add(worker);               // rate-limit -> rotate out
        return wiki.get(url);
    }
    static void crawl(String seed) {
        frontier.add(seed);
        while (!frontier.isEmpty()) {
            String url = frontier.poll();
            if (visited.contains(url)) continue;
            visited.add(url);
            String worker = pickWorker();
            Object[] page = fetch(worker, url);
            String content = (String) page[0];
            String[] links = (String[]) page[1];
            db.put(url, new long[]{fnv1a(content), order.size()});
            order.add(url);
            for (String l : links) if (!visited.contains(l)) frontier.add(l);
        }
    }
    static long[] recrawl(String url) {
        Object[] page = wiki.get(url);
        long nh = fnv1a((String) page[0]);
        long oh = db.get(url)[0];
        if (nh != oh) {
            db.put(url, new long[]{nh, db.get(url)[1]});
            return new long[]{oh, nh};
        }
        return null;
    }

    public static void main(String[] args) {
        wiki.put("Main", new Object[]{"Welcome to the wiki", new String[]{"A", "B"}});
        wiki.put("A", new Object[]{"Page A content", new String[]{"C"}});
        wiki.put("B", new Object[]{"Page B content", new String[]{"C"}});
        wiki.put("C", new Object[]{"Page C content", new String[]{"Main"}});

        crawl("Main");
        System.out.println("Crawled " + order.size() + " pages: [" + String.join(", ", order) + "]");

        wiki.put("C", new Object[]{"Page C content (edited 2026)", new String[]{"Main"}});
        long[] res = recrawl("C");
        if (res != null)
            System.out.printf("Re-crawl detected update on 'C': hash %08x -> %08x, re-stored.%n", res[0], res[1]);
    }
}
