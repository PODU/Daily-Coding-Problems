// LFU cache: HashMap key->node, HashMap freq->doubly-linked-list, track minFreq. O(1) per op.
// Time: O(1) get/set. Space: O(capacity).
import java.util.HashMap;

public class Solution {

    static class Node {
        int key, val, freq = 1;
        Node prev, next;
        Node(int k, int v) { key = k; val = v; }
    }

    static class DLL {
        Node head = new Node(0, 0), tail = new Node(0, 0);
        int size = 0;
        DLL() { head.next = tail; tail.prev = head; }
        void addFront(Node n) {
            n.prev = head; n.next = head.next;
            head.next.prev = n; head.next = n; size++;
        }
        void remove(Node n) {
            n.prev.next = n.next; n.next.prev = n.prev; size--;
        }
        Node removeLast() {
            if (size == 0) return null;
            Node n = tail.prev; remove(n); return n;
        }
    }

    static class LFUCache {
        int cap, minFreq = 0;
        HashMap<Integer, Node> nodes = new HashMap<>();
        HashMap<Integer, DLL> freqs = new HashMap<>();

        LFUCache(int capacity) { cap = capacity; }

        void touch(Node node) {
            int f = node.freq;
            DLL dll = freqs.get(f);
            dll.remove(node);
            if (dll.size == 0) {
                freqs.remove(f);
                if (minFreq == f) minFreq++;
            }
            node.freq++;
            freqs.computeIfAbsent(node.freq, k -> new DLL()).addFront(node);
        }

        Integer get(int key) {
            Node node = nodes.get(key);
            if (node == null) return null;
            touch(node);
            return node.val;
        }

        void set(int key, int value) {
            if (cap <= 0) return;
            Node node = nodes.get(key);
            if (node != null) {
                node.val = value;
                touch(node);
                return;
            }
            if (nodes.size() >= cap) {
                Node lru = freqs.get(minFreq).removeLast();
                nodes.remove(lru.key);
            }
            Node n = new Node(key, value);
            nodes.put(key, n);
            freqs.computeIfAbsent(1, k -> new DLL()).addFront(n);
            minFreq = 1;
        }
    }

    static String fmt(Integer x) { return x == null ? "null" : x.toString(); }

    public static void main(String[] args) {
        LFUCache cache = new LFUCache(2);
        cache.set(1, 1);
        cache.set(2, 2);
        System.out.println(fmt(cache.get(1))); // 1
        cache.set(3, 3); // evicts key 2
        System.out.println(fmt(cache.get(2))); // null
        cache.get(3); // access key 3 (raises its freq) so key 1 becomes LFU/LRU victim
        cache.set(4, 4); // evicts key 1
        System.out.println(fmt(cache.get(1))); // null
        System.out.println(fmt(cache.get(3))); // 3
        System.out.println(fmt(cache.get(4))); // 4
    }
}
