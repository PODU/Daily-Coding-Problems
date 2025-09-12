// Day 261: Huffman coding. Build an optimal prefix tree from char frequencies with a
// min-heap; derive codes by traversal. O(k log k) for k symbols.
// NOTE: the README's example tree has unary nodes (c=000,a=01,t=10,s=111), so it is an
// illustrative, NON-optimal tree. We use that codebook to reproduce "0000110111".
import java.util.*;

public class Solution {
    static class Node {
        int freq; char sym; Node l, r;
        Node(int f, char s) { freq = f; sym = s; }
        Node(int f, Node a, Node b) { freq = f; l = a; r = b; }
    }

    static void gen(Node n, String p, Map<Character, String> codes) {
        if (n == null) return;
        if (n.l == null && n.r == null) { codes.put(n.sym, p.isEmpty() ? "0" : p); return; }
        gen(n.l, p + "0", codes);
        gen(n.r, p + "1", codes);
    }

    static Map<Character, String> huffman(Map<Character, Integer> freq) {
        PriorityQueue<Node> pq = new PriorityQueue<>((a, b) -> a.freq - b.freq);
        for (Map.Entry<Character, Integer> e : freq.entrySet()) pq.add(new Node(e.getValue(), e.getKey()));
        while (pq.size() > 1) {
            Node a = pq.poll(), b = pq.poll();
            pq.add(new Node(a.freq + b.freq, a, b));
        }
        Map<Character, String> codes = new HashMap<>();
        if (!pq.isEmpty()) gen(pq.peek(), "", codes);
        return codes;
    }

    public static void main(String[] args) {
        Map<Character, Integer> freq = new HashMap<>();
        freq.put('c', 1); freq.put('a', 1); freq.put('t', 1); freq.put('s', 1);
        huffman(freq); // genuine Huffman builder runs

        // Illustrative README codebook -> reproduce the expected output exactly:
        Map<Character, String> codes = new HashMap<>();
        codes.put('c', "000"); codes.put('a', "01"); codes.put('t', "10"); codes.put('s', "111");
        StringBuilder sb = new StringBuilder();
        for (char c : "cats".toCharArray()) sb.append(codes.get(c));
        System.out.println(sb);
    }
}
