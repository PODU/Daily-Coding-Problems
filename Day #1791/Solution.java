// Day 1791: All O(1) data structure (plus / minus / get_max / get_min).
//
// Idea: a doubly linked list of "buckets", one per distinct count, kept
// sorted by count. Each bucket owns the set of keys sitting at that count,
// and a HashMap points every key at its current bucket. Incrementing or
// decrementing a key only moves it to a neighbouring bucket, so each
// operation touches a constant number of nodes. Min and max are the
// buckets right next to the sentinels.
import java.util.*;

public class Solution {

    /** A node in the linked list: one count, and every key that has it. */
    static class Bucket {
        int count;
        Set<String> keys = new HashSet<>();
        Bucket prev, next;

        Bucket(int c) {
            count = c;
        }
    }

    static class AllOne {
        // Sentinels at both ends mean we never special-case front/back inserts.
        Bucket head = new Bucket(Integer.MIN_VALUE);
        Bucket tail = new Bucket(Integer.MAX_VALUE);
        Map<String, Bucket> keyBucket = new HashMap<>(); // key -> its bucket

        AllOne() {
            head.next = tail;
            tail.prev = head;
        }

        /** Splice a fresh bucket for {@code count} right after {@code node}. */
        Bucket insertAfter(Bucket node, int count) {
            Bucket b = new Bucket(count);
            b.prev = node;
            b.next = node.next;
            node.next.prev = b;
            node.next = b;
            return b;
        }

        /** Unlink an empty bucket from the list. */
        void removeBucket(Bucket b) {
            b.prev.next = b.next;
            b.next.prev = b.prev;
        }

        void plus(String key) {
            if (keyBucket.containsKey(key)) {
                // Existing key: shift it one bucket to the right (count + 1).
                Bucket cur = keyBucket.get(key);
                Bucket nxt = cur.next;
                if (nxt == tail || nxt.count != cur.count + 1) {
                    // No bucket for count+1 yet, so make one next door.
                    nxt = insertAfter(cur, cur.count + 1);
                }
                nxt.keys.add(key);
                keyBucket.put(key, nxt);
                cur.keys.remove(key);
                if (cur.keys.isEmpty()) {
                    removeBucket(cur); // the bucket we left is empty now
                }
            } else {
                // New key: it belongs in the count-1 bucket at the front.
                Bucket first = head.next;
                if (first == tail || first.count != 1) {
                    first = insertAfter(head, 1);
                }
                first.keys.add(key);
                keyBucket.put(key, first);
            }
        }

        void minus(String key) {
            if (!keyBucket.containsKey(key)) {
                return; // unknown key -> no-op
            }

            Bucket cur = keyBucket.get(key);
            cur.keys.remove(key);

            if (cur.count == 1) {
                // Count would hit zero, so the key disappears entirely.
                keyBucket.remove(key);
            } else {
                // Shift the key one bucket to the left (count - 1).
                Bucket prv = cur.prev;
                if (prv == head || prv.count != cur.count - 1) {
                    prv = insertAfter(cur.prev, cur.count - 1);
                }
                prv.keys.add(key);
                keyBucket.put(key, prv);
            }

            if (cur.keys.isEmpty()) {
                removeBucket(cur);
            }
        }

        // Any key from the end buckets is a valid answer; taking the
        // lexicographically smallest keeps the output deterministic.
        String getMax() {
            return tail.prev == head ? "" : Collections.min(tail.prev.keys);
        }

        String getMin() {
            return head.next == tail ? "" : Collections.min(head.next.keys);
        }
    }

    public static void main(String[] args) {
        AllOne a = new AllOne();
        a.plus("apple");
        a.plus("apple");
        a.plus("banana");
        System.out.println("max=" + a.getMax() + " min=" + a.getMin()); // apple / banana

        a.plus("banana");
        a.plus("banana");
        System.out.println("max=" + a.getMax() + " min=" + a.getMin()); // banana / apple

        a.minus("apple");
        a.minus("apple");
        System.out.println("max=" + a.getMax() + " min=" + a.getMin()); // banana / banana
    }
}
