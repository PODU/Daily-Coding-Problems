// All O(1) data structure (LeetCode 432).
// Doubly linked list of count-buckets (each holds a set of keys) + hashmap key->bucket. All ops O(1); space O(N).
import java.util.*;

public class Solution {
    static class Bucket {
        int count;
        Set<String> keys = new HashSet<>();
        Bucket prev, next;
        Bucket(int c) { count = c; }
    }

    static class AllOne {
        Bucket head, tail;
        Map<String, Bucket> keyBucket = new HashMap<>();

        AllOne() {
            head = new Bucket(Integer.MIN_VALUE);
            tail = new Bucket(Integer.MAX_VALUE);
            head.next = tail; tail.prev = head;
        }
        Bucket insertAfter(Bucket node, int count) {
            Bucket b = new Bucket(count);
            b.prev = node; b.next = node.next;
            node.next.prev = b; node.next = b;
            return b;
        }
        void removeBucket(Bucket b) {
            b.prev.next = b.next; b.next.prev = b.prev;
        }
        void plus(String key) {
            if (keyBucket.containsKey(key)) {
                Bucket cur = keyBucket.get(key);
                Bucket nxt = cur.next;
                if (nxt == tail || nxt.count != cur.count + 1)
                    nxt = insertAfter(cur, cur.count + 1);
                nxt.keys.add(key); keyBucket.put(key, nxt);
                cur.keys.remove(key);
                if (cur.keys.isEmpty()) removeBucket(cur);
            } else {
                Bucket first = head.next;
                if (first == tail || first.count != 1)
                    first = insertAfter(head, 1);
                first.keys.add(key); keyBucket.put(key, first);
            }
        }
        void minus(String key) {
            if (!keyBucket.containsKey(key)) return;
            Bucket cur = keyBucket.get(key);
            if (cur.count == 1) {
                keyBucket.remove(key);
            } else {
                Bucket prv = cur.prev;
                if (prv == head || prv.count != cur.count - 1)
                    prv = insertAfter(cur.prev, cur.count - 1);
                prv.keys.add(key); keyBucket.put(key, prv);
            }
            cur.keys.remove(key);
            if (cur.keys.isEmpty()) removeBucket(cur);
        }
        String getMax() { return tail.prev == head ? "" : Collections.min(tail.prev.keys); }
        String getMin() { return head.next == tail ? "" : Collections.min(head.next.keys); }
    }

    public static void main(String[] args) {
        AllOne a = new AllOne();
        a.plus("a"); a.plus("a"); a.plus("a");
        a.plus("b");
        System.out.println("get_max: " + a.getMax());
        System.out.println("get_min: " + a.getMin());
        a.minus("a"); a.minus("a");
        System.out.println("get_max: " + a.getMax());
        System.out.println("get_min: " + a.getMin());
    }
}
