// LRU cache via HashMap + doubly linked list. O(1) per get/set. Space O(capacity).
import java.util.HashMap;

public class Solution {
    static class Node {
        int key, val;
        Node prev, next;
        Node(int k, int v) { key = k; val = v; }
    }

    static class LRUCache {
        int cap;
        HashMap<Integer, Node> map = new HashMap<>();
        Node head, tail; // head = MRU sentinel, tail = LRU sentinel

        LRUCache(int n) {
            cap = n;
            head = new Node(0, 0);
            tail = new Node(0, 0);
            head.next = tail;
            tail.prev = head;
        }

        private void remove(Node node) {
            node.prev.next = node.next;
            node.next.prev = node.prev;
        }

        private void addFront(Node node) {
            node.next = head.next;
            node.prev = head;
            head.next.prev = node;
            head.next = node;
        }

        Integer get(int key) {
            Node node = map.get(key);
            if (node == null) return null;
            remove(node);
            addFront(node);
            return node.val;
        }

        void set(int key, int value) {
            Node node = map.get(key);
            if (node != null) {
                node.val = value;
                remove(node);
                addFront(node);
                return;
            }
            if (map.size() == cap) {
                Node lru = tail.prev;
                remove(lru);
                map.remove(lru.key);
            }
            Node fresh = new Node(key, value);
            addFront(fresh);
            map.put(key, fresh);
        }
    }

    static void printGet(LRUCache c, int key) {
        Integer v = c.get(key);
        System.out.println(v == null ? "null" : v.toString());
    }

    public static void main(String[] args) {
        LRUCache c = new LRUCache(2);
        c.set(1, 1);
        c.set(2, 2);
        printGet(c, 1);   // 1
        c.set(3, 3);      // evicts 2
        printGet(c, 2);   // null
        c.set(4, 4);      // evicts 1
        printGet(c, 1);   // null
        printGet(c, 3);   // 3
        printGet(c, 4);   // 4
    }
}
