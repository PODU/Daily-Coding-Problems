// Day 848: LRU cache with O(1) get/set using a hash map + doubly linked list.
// Map key -> node; most-recent near head, evict tail. O(1) per op.
import java.util.*;

public class Solution {
    static class Node {
        int key, val; Node prev, next;
        Node(int k, int v){ key = k; val = v; }
    }
    static class LRUCache {
        int cap;
        Map<Integer, Node> map = new HashMap<>();
        Node head, tail;                 // head = MRU sentinel side
        LRUCache(int n){
            cap = n;
            head = new Node(0,0); tail = new Node(0,0);
            head.next = tail; tail.prev = head;
        }
        private void remove(Node n){ n.prev.next = n.next; n.next.prev = n.prev; }
        private void addFront(Node n){
            n.next = head.next; n.prev = head;
            head.next.prev = n; head.next = n;
        }
        Integer get(int key){
            Node n = map.get(key);
            if(n == null) return null;
            remove(n); addFront(n);
            return n.val;
        }
        void set(int key, int value){
            Node n = map.get(key);
            if(n != null){ n.val = value; remove(n); addFront(n); return; }
            if(map.size() == cap){
                Node lru = tail.prev;
                remove(lru); map.remove(lru.key);
            }
            Node nn = new Node(key, value);
            addFront(nn); map.put(key, nn);
        }
    }

    public static void main(String[] args){
        LRUCache c = new LRUCache(2);
        c.set(1,1);
        c.set(2,2);
        System.out.println(c.get(1)); // 1
        c.set(3,3);                    // evicts 2
        System.out.println(c.get(2)); // null
        c.set(4,4);                    // evicts 1
        System.out.println(c.get(1)); // null
        System.out.println(c.get(3)); // 3
        System.out.println(c.get(4)); // 4
    }
}
