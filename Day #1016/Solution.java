// Huffman coding: min-heap repeatedly merges two smallest nodes, then DFS assigns codes (left='0', right='1').
// Tie-break by insertion order for determinism. O(k log k) time, O(k) space.
import java.util.*;

public class Solution {
    static class Node {
        int freq, order; char ch; Node l, r;
        Node(int f, int o, char c) { freq = f; order = o; ch = c; }
    }

    static void dfs(Node n, String code, Map<Character,String> codes) {
        if (n.l == null && n.r == null) { codes.put(n.ch, code.isEmpty() ? "0" : code); return; }
        dfs(n.l, code + "0", codes);
        dfs(n.r, code + "1", codes);
    }

    static Map<Character,String> huffman(Map<Character,Integer> freqs) {
        PriorityQueue<Node> pq = new PriorityQueue<>(
            (a, b) -> a.freq != b.freq ? a.freq - b.freq : a.order - b.order);
        int[] order = {0};
        for (char ch : new TreeSet<>(freqs.keySet()))
            pq.add(new Node(freqs.get(ch), order[0]++, ch));
        while (pq.size() > 1) {
            Node a = pq.poll(), b = pq.poll();
            Node p = new Node(a.freq + b.freq, order[0]++, '\0');
            p.l = a; p.r = b;
            pq.add(p);
        }
        Map<Character,String> codes = new TreeMap<>();
        dfs(pq.poll(), "", codes);
        return codes;
    }

    public static void main(String[] args) {
        Map<Character,Integer> freqs = new HashMap<>();
        freqs.put('a',5); freqs.put('b',9); freqs.put('c',12);
        freqs.put('d',13); freqs.put('e',16); freqs.put('f',45);
        Map<Character,String> codes = huffman(freqs);
        for (Map.Entry<Character,String> e : codes.entrySet())
            System.out.println(e.getKey() + ": " + e.getValue());
    }
}
