// Huffman coding: build tree via min-heap, merge two smallest nodes, assign 0/1 edges.
// Time: O(n log n) for n symbols. Space: O(n).
import java.util.*;

public class Solution {
    static class Node {
        char ch; long freq; int order; Node l, r;
        Node(char c, long f, int o) { ch = c; freq = f; order = o; }
    }

    static void build(Node n, String code, TreeMap<Character,String> out) {
        if (n == null) return;
        if (n.l == null && n.r == null) { out.put(n.ch, code.isEmpty() ? "0" : code); return; }
        build(n.l, code + "0", out);
        build(n.r, code + "1", out);
    }

    public static void main(String[] args) {
        char[] chars = {'a','b','c','d','e','f'};
        long[] freqs = {5,9,12,13,16,45};
        PriorityQueue<Node> pq = new PriorityQueue<>((a,b) ->
            a.freq != b.freq ? Long.compare(a.freq, b.freq) : Integer.compare(a.order, b.order));
        int counter = 0;
        Map<Character,Long> fmap = new HashMap<>();
        for (int i = 0; i < chars.length; i++) {
            pq.add(new Node(chars[i], freqs[i], counter++));
            fmap.put(chars[i], freqs[i]);
        }
        while (pq.size() > 1) {
            Node a = pq.poll(), b = pq.poll();
            Node m = new Node('\0', a.freq + b.freq, counter++);
            m.l = a; m.r = b;
            pq.add(m);
        }
        Node root = pq.poll();
        TreeMap<Character,String> codes = new TreeMap<>();
        build(root, "", codes);
        long totalBits = 0;
        for (Map.Entry<Character,String> e : codes.entrySet()) {
            System.out.println(e.getKey() + ": " + e.getValue());
            totalBits += (long) e.getValue().length() * fmap.get(e.getKey());
        }
        System.out.println("Total encoded bits: " + totalBits);
    }
}
