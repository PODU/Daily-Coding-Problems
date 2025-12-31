// Day 829: O(1) data structure with plus/minus/get_max/get_min.
// Doubly-linked list of count-buckets (each a set of keys) + key->bucket map.
// All operations O(1) time; O(K) space for K distinct keys.
import java.util.*;

public class Solution {
    static class Bucket {
        long count;
        Set<String> keys = new HashSet<>();
        Bucket prev, next;
        Bucket(long count) { this.count = count; }
    }

    static class AllOne {
        Bucket head, tail;            // sentinels
        Map<String, Bucket> keyBucket = new HashMap<>();

        AllOne() {
            head = new Bucket(Long.MIN_VALUE);
            tail = new Bucket(Long.MAX_VALUE);
            head.next = tail;
            tail.prev = head;
        }

        Bucket insertAfter(Bucket node, long count) {
            Bucket b = new Bucket(count);
            b.prev = node;
            b.next = node.next;
            node.next.prev = b;
            node.next = b;
            return b;
        }

        void remove(Bucket node) {
            node.prev.next = node.next;
            node.next.prev = node.prev;
        }

        void plus(String key) {
            if (keyBucket.containsKey(key)) {
                Bucket cur = keyBucket.get(key);
                long newCount = cur.count + 1;
                Bucket nxt = cur.next;
                if (nxt.count != newCount) nxt = insertAfter(cur, newCount);
                nxt.keys.add(key);
                keyBucket.put(key, nxt);
                cur.keys.remove(key);
                if (cur.keys.isEmpty()) remove(cur);
            } else {
                Bucket first = head.next;
                if (first.count != 1) first = insertAfter(head, 1);
                first.keys.add(key);
                keyBucket.put(key, first);
            }
        }

        void minus(String key) {
            if (!keyBucket.containsKey(key)) return;
            Bucket cur = keyBucket.get(key);
            if (cur.count == 1) {
                cur.keys.remove(key);
                keyBucket.remove(key);
                if (cur.keys.isEmpty()) remove(cur);
                return;
            }
            long newCount = cur.count - 1;
            Bucket prv = cur.prev;
            if (prv.count != newCount) prv = insertAfter(cur.prev, newCount);
            prv.keys.add(key);
            keyBucket.put(key, prv);
            cur.keys.remove(key);
            if (cur.keys.isEmpty()) remove(cur);
        }

        String getMax() {
            if (tail.prev == head) return "";
            return Collections.min(tail.prev.keys);
        }

        String getMin() {
            if (head.next == tail) return "";
            return Collections.min(head.next.keys);
        }
    }

    public static void main(String[] args) {
        AllOne ao = new AllOne();
        ao.plus("a");
        ao.plus("b");
        ao.plus("b");
        System.out.println("get_max: " + ao.getMax());  // b
        System.out.println("get_min: " + ao.getMin());  // a
        ao.minus("b");
        ao.minus("b");
        System.out.println("get_max: " + ao.getMax());  // a
    }
}
