// All O(1) structure: doubly-linked list of count-buckets (increasing), each holds a key set; hashmap key->bucket.
// plus/minus move key to adjacent bucket. get_max=last bucket, get_min=first bucket. All O(1).
import java.util.*;

public class Solution {
    static class Bucket {
        int count;
        LinkedHashSet<String> keys = new LinkedHashSet<>();
        Bucket prev, next;
        Bucket(int c) { count = c; }
    }

    static class AllOne {
        Bucket head, tail; // head = smallest count, tail = largest
        Map<String, Bucket> keyBucket = new HashMap<>();

        AllOne() {
            head = new Bucket(Integer.MIN_VALUE);
            tail = new Bucket(Integer.MAX_VALUE);
            head.next = tail; tail.prev = head;
        }

        private Bucket insertAfter(Bucket b, int count) {
            Bucket nb = new Bucket(count);
            nb.prev = b; nb.next = b.next;
            b.next.prev = nb; b.next = nb;
            return nb;
        }
        private void remove(Bucket b) {
            b.prev.next = b.next; b.next.prev = b.prev;
        }

        void plus(String key) {
            if (!keyBucket.containsKey(key)) {
                Bucket first = head.next;
                if (first == tail || first.count != 1) first = insertAfter(head, 1);
                first.keys.add(key);
                keyBucket.put(key, first);
                return;
            }
            Bucket cur = keyBucket.get(key);
            Bucket nxt = cur.next;
            if (nxt == tail || nxt.count != cur.count + 1) nxt = insertAfter(cur, cur.count + 1);
            nxt.keys.add(key);
            keyBucket.put(key, nxt);
            cur.keys.remove(key);
            if (cur.keys.isEmpty()) remove(cur);
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
            Bucket prv = cur.prev;
            if (prv == head || prv.count != cur.count - 1) prv = insertAfter(cur.prev, cur.count - 1);
            prv.keys.add(key);
            keyBucket.put(key, prv);
            cur.keys.remove(key);
            if (cur.keys.isEmpty()) remove(cur);
        }

        String getMax() { return tail.prev == head ? "" : Collections.min(tail.prev.keys); }
        String getMin() { return head.next == tail ? "" : Collections.min(head.next.keys); }
    }

    public static void main(String[] args) {
        AllOne a = new AllOne();
        a.plus("a"); a.plus("b"); a.plus("b");
        a.plus("c"); a.plus("c"); a.plus("c");
        System.out.println("max=" + a.getMax());
        System.out.println("min=" + a.getMin());
        a.minus("c"); a.minus("c");
        System.out.println("max=" + a.getMax());
    }
}
