// Day 697: LRU cache with O(1) get/set.
// Approach: hash map + doubly linked list ordered by recency.
// get/set O(1) time, O(n) space.
import java.util.*;

public class Solution {
    static class LRUCache {
        class Node { int k, v; Node prev, next; Node(int k, int v) { this.k = k; this.v = v; } }
        int cap;
        Map<Integer, Node> map = new HashMap<>();
        Node head = new Node(0, 0), tail = new Node(0, 0); // head=MRU sentinel, tail=LRU sentinel

        LRUCache(int n) { cap = n; head.next = tail; tail.prev = head; }

        void remove(Node n) { n.prev.next = n.next; n.next.prev = n.prev; }
        void addFront(Node n) { n.next = head.next; n.prev = head; head.next.prev = n; head.next = n; }

        Integer get(int key) {
            Node n = map.get(key);
            if (n == null) return null;
            remove(n); addFront(n);
            return n.v;
        }

        void set(int key, int value) {
            Node n = map.get(key);
            if (n != null) { n.v = value; remove(n); addFront(n); return; }
            if (map.size() == cap) {
                Node lru = tail.prev;
                remove(lru); map.remove(lru.k);
            }
            Node nn = new Node(key, value);
            addFront(nn); map.put(key, nn);
        }
    }

    public static void main(String[] args) {
        LRUCache c = new LRUCache(2);
        c.set(1, 1); c.set(2, 2);
        System.out.println(c.get(1)); // 1
        c.set(3, 3);                  // evicts key 2
        System.out.println(c.get(2)); // null
        System.out.println(c.get(3)); // 3
    }
}
