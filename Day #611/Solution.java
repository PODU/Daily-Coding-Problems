// Day 611: All O(1) structure (plus / minus / get_max / get_min).
// Approach: doubly-linked list of value-buckets (set of keys) + key->bucket map. All ops O(1).
import java.util.*;

public class Solution {
    static class Bucket {
        int value;
        Set<String> keys = new HashSet<>();
        Bucket prev, next;
        Bucket(int v) { value = v; }
    }

    static class AllOne {
        Bucket head = new Bucket(0), tail = new Bucket(0);
        Map<String, Bucket> keyToBucket = new HashMap<>();

        AllOne() { head.next = tail; tail.prev = head; }

        Bucket insertAfter(Bucket node, int value) {
            Bucket b = new Bucket(value);
            b.prev = node; b.next = node.next;
            node.next.prev = b; node.next = b;
            return b;
        }
        void removeBucket(Bucket node) {
            node.prev.next = node.next;
            node.next.prev = node.prev;
        }
        void plus(String key) {
            if (keyToBucket.containsKey(key)) {
                Bucket cur = keyToBucket.get(key);
                Bucket nxt = cur.next;
                if (nxt == tail || nxt.value != cur.value + 1) nxt = insertAfter(cur, cur.value + 1);
                nxt.keys.add(key);
                keyToBucket.put(key, nxt);
                cur.keys.remove(key);
                if (cur.keys.isEmpty()) removeBucket(cur);
            } else {
                Bucket first = head.next;
                if (first == tail || first.value != 1) first = insertAfter(head, 1);
                first.keys.add(key);
                keyToBucket.put(key, first);
            }
        }
        void minus(String key) {
            if (!keyToBucket.containsKey(key)) return;
            Bucket cur = keyToBucket.get(key);
            if (cur.value == 1) {
                keyToBucket.remove(key);
            } else {
                Bucket prv = cur.prev;
                if (prv == head || prv.value != cur.value - 1) prv = insertAfter(cur.prev, cur.value - 1);
                prv.keys.add(key);
                keyToBucket.put(key, prv);
            }
            cur.keys.remove(key);
            if (cur.keys.isEmpty()) removeBucket(cur);
        }
        String getMax() { return tail.prev == head ? "" : Collections.min(tail.prev.keys); }
        String getMin() { return head.next == tail ? "" : Collections.min(head.next.keys); }
    }

    public static void main(String[] args) {
        AllOne a = new AllOne();
        a.plus("a"); a.plus("b"); a.plus("a"); // a=2, b=1
        System.out.println(a.getMax()); // a
        System.out.println(a.getMin()); // b
    }
}
