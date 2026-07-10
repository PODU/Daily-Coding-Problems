// Day 1791: All O(1) data structure (plus / minus / get_max / get_min).
//
// Idea: a doubly linked list of "buckets", one per distinct count, kept
// sorted by count. Each bucket owns the set of keys sitting at that count,
// and a hash map points every key at its current bucket. Incrementing or
// decrementing a key only moves it to a neighbouring bucket, so each
// operation touches a constant number of nodes. Min and max are the
// buckets right next to the sentinels.
#include <bits/stdc++.h>
using namespace std;

// A node in the linked list: one count, plus every key that has it.
struct Bucket {
    int count;
    unordered_set<string> keys;
    Bucket *prev, *next;
    Bucket(int c) : count(c), prev(nullptr), next(nullptr) {}
};

class AllOne {
    Bucket *head, *tail; // sentinels; head->next = smallest, tail->prev = largest
    unordered_map<string, Bucket*> keyBucket; // key -> its current bucket

    // Splice a fresh bucket for `count` right after `node`.
    Bucket* insertAfter(Bucket* node, int count) {
        Bucket* b = new Bucket(count);
        b->prev = node;
        b->next = node->next;
        node->next->prev = b;
        node->next = b;
        return b;
    }

    // Unlink an empty bucket from the list and free it.
    void removeBucket(Bucket* b) {
        b->prev->next = b->next;
        b->next->prev = b->prev;
        delete b;
    }

public:
    AllOne() {
        head = new Bucket(INT_MIN);
        tail = new Bucket(INT_MAX);
        head->next = tail;
        tail->prev = head;
    }

    void plus(const string& key) {
        if (keyBucket.count(key)) {
            // Existing key: shift it one bucket to the right (count + 1).
            Bucket* cur = keyBucket[key];
            Bucket* nxt = cur->next;
            if (nxt == tail || nxt->count != cur->count + 1) {
                // No bucket for count+1 yet, so make one next door.
                nxt = insertAfter(cur, cur->count + 1);
            }
            nxt->keys.insert(key);
            keyBucket[key] = nxt;
            cur->keys.erase(key);
            if (cur->keys.empty()) {
                removeBucket(cur); // the bucket we left is empty now
            }
        } else {
            // New key: it belongs in the count-1 bucket at the front.
            Bucket* first = head->next;
            if (first == tail || first->count != 1) {
                first = insertAfter(head, 1);
            }
            first->keys.insert(key);
            keyBucket[key] = first;
        }
    }

    void minus(const string& key) {
        if (!keyBucket.count(key)) return; // unknown key -> no-op

        Bucket* cur = keyBucket[key];
        cur->keys.erase(key);

        if (cur->count == 1) {
            // Count would hit zero, so the key disappears entirely.
            keyBucket.erase(key);
        } else {
            // Shift the key one bucket to the left (count - 1).
            Bucket* prv = cur->prev;
            if (prv == head || prv->count != cur->count - 1) {
                prv = insertAfter(cur->prev, cur->count - 1);
            }
            prv->keys.insert(key);
            keyBucket[key] = prv;
        }

        if (cur->keys.empty()) {
            removeBucket(cur);
        }
    }

    // Any key from the end buckets is a valid answer; taking the
    // lexicographically smallest keeps the output deterministic.
    string get_max() {
        if (tail->prev == head) return "";
        return *min_element(tail->prev->keys.begin(), tail->prev->keys.end());
    }

    string get_min() {
        if (head->next == tail) return "";
        return *min_element(head->next->keys.begin(), head->next->keys.end());
    }
};

int main() {
    AllOne a;
    a.plus("apple");
    a.plus("apple");
    a.plus("banana");
    cout << "max=" << a.get_max() << " min=" << a.get_min() << "\n"; // apple / banana

    a.plus("banana");
    a.plus("banana");
    cout << "max=" << a.get_max() << " min=" << a.get_min() << "\n"; // banana / apple

    a.minus("apple");
    a.minus("apple");
    cout << "max=" << a.get_max() << " min=" << a.get_min() << "\n"; // banana / banana
    return 0;
}
