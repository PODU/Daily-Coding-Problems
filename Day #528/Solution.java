// Day 528: Huffman coding. Build a prefix tree by repeatedly merging the two
// lowest-frequency nodes (min-heap), then read each char's code from its
// root-to-leaf path (left=0, right=1). Time O(n log n), space O(n).
// Note: the README's example tree is illustrative, not a strict Huffman tree;
// this produces a correct, deterministic optimal-prefix Huffman mapping.
import java.util.*;

public class Solution {
    static class Node {
        long freq;
        int order;
        char ch;
        Node l, r;
        Node(long f, int o, char c, Node l, Node r) { freq = f; order = o; ch = c; this.l = l; this.r = r; }
    }

    static void buildCodes(Node n, String path, Map<Character, String> codes) {
        if (n.l == null && n.r == null) {
            codes.put(n.ch, path.isEmpty() ? "0" : path);
            return;
        }
        buildCodes(n.l, path + "0", codes);
        buildCodes(n.r, path + "1", codes);
    }

    public static void main(String[] args) {
        char[] chars = {'c', 'a', 't', 's'};
        long[] freqs = {1, 1, 1, 1};

        PriorityQueue<Node> pq = new PriorityQueue<>((a, b) ->
                a.freq != b.freq ? Long.compare(a.freq, b.freq) : Integer.compare(a.order, b.order));
        int order = 0;
        for (int i = 0; i < chars.length; i++) pq.add(new Node(freqs[i], order++, chars[i], null, null));

        while (pq.size() > 1) {
            Node a = pq.poll(), b = pq.poll();
            pq.add(new Node(a.freq + b.freq, order++, '\0', a, b));
        }
        Node root = pq.poll();

        Map<Character, String> codes = new TreeMap<>();
        buildCodes(root, "", codes);
        for (Map.Entry<Character, String> e : codes.entrySet())
            System.out.println(e.getKey() + " -> " + e.getValue());

        String word = "cats";
        StringBuilder encoded = new StringBuilder();
        for (char c : word.toCharArray()) encoded.append(codes.get(c));
        System.out.println(word + " -> " + encoded);
    }
}
