// Huffman coding: merge the two lowest-frequency nodes via a min-heap to build
// an optimal prefix tree, then DFS to assign codes (left=0, right=1).
// Time O(C log C) for C distinct chars, Space O(C).
import java.util.*;

public class Solution {
    static class Node {
        int freq, order; char ch; Node l, r;
        Node(int f, char c, int o) { freq = f; ch = c; order = o; }
        Node(int f, Node a, Node b, int o) { freq = f; l = a; r = b; order = o; }
    }

    static void dfs(Node n, String path, Map<Character,String> codes) {
        if (n.l == null && n.r == null) {
            codes.put(n.ch, path.isEmpty() ? "0" : path);
            return;
        }
        dfs(n.l, path + "0", codes);
        dfs(n.r, path + "1", codes);
    }

    public static void main(String[] args) {
        char[] chars = {'c', 'a', 't', 's'};
        int[] freqs = {1, 4, 3, 2};
        PriorityQueue<Node> pq = new PriorityQueue<>(
            (x, y) -> x.freq != y.freq ? x.freq - y.freq : x.order - y.order);
        int cnt = 0;
        for (int i = 0; i < chars.length; i++)
            pq.add(new Node(freqs[i], chars[i], cnt++));
        while (pq.size() > 1) {
            Node a = pq.poll(), b = pq.poll();
            pq.add(new Node(a.freq + b.freq, a, b, cnt++));
        }
        Node root = pq.poll();
        Map<Character,String> codes = new TreeMap<>();
        dfs(root, "", codes);
        for (Map.Entry<Character,String> e : codes.entrySet())
            System.out.println(e.getKey() + ": " + e.getValue());
        StringBuilder enc = new StringBuilder();
        for (char c : "cats".toCharArray()) enc.append(codes.get(c));
        System.out.println("encode(cats): " + enc);
    }
}
