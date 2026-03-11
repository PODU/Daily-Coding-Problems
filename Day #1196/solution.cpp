// All O(1) data structure (LeetCode 432).
// Doubly linked list of count-buckets (each holds a set of keys) + hashmap key->bucket. All ops O(1); space O(N).
#include <bits/stdc++.h>
using namespace std;

struct Bucket {
    int count;
    unordered_set<string> keys;
    Bucket *prev = nullptr, *next = nullptr;
    Bucket(int c) : count(c) {}
};

class AllOne {
    Bucket *head, *tail; // sentinels: head(min side) ... tail(max side)
    unordered_map<string, Bucket*> keyBucket;

    Bucket* insertAfter(Bucket* node, int count) {
        Bucket* b = new Bucket(count);
        b->prev = node; b->next = node->next;
        node->next->prev = b; node->next = b;
        return b;
    }
    void removeBucket(Bucket* b) {
        b->prev->next = b->next; b->next->prev = b->prev;
        delete b;
    }
public:
    AllOne() {
        head = new Bucket(INT_MIN); tail = new Bucket(INT_MAX);
        head->next = tail; tail->prev = head;
    }
    void plus(const string& key) {
        if (keyBucket.count(key)) {
            Bucket* cur = keyBucket[key];
            Bucket* nxt = cur->next;
            if (nxt == tail || nxt->count != cur->count + 1)
                nxt = insertAfter(cur, cur->count + 1);
            nxt->keys.insert(key); keyBucket[key] = nxt;
            cur->keys.erase(key);
            if (cur->keys.empty()) removeBucket(cur);
        } else {
            Bucket* first = head->next;
            if (first == tail || first->count != 1)
                first = insertAfter(head, 1);
            first->keys.insert(key); keyBucket[key] = first;
        }
    }
    void minus(const string& key) {
        if (!keyBucket.count(key)) return;
        Bucket* cur = keyBucket[key];
        if (cur->count == 1) {
            keyBucket.erase(key);
        } else {
            Bucket* prv = cur->prev;
            if (prv == head || prv->count != cur->count - 1)
                prv = insertAfter(cur->prev, cur->count - 1);
            prv->keys.insert(key); keyBucket[key] = prv;
        }
        cur->keys.erase(key);
        if (cur->keys.empty()) removeBucket(cur);
    }
    string getMax() { return tail->prev == head ? "" : *min_element(tail->prev->keys.begin(), tail->prev->keys.end()); }
    string getMin() { return head->next == tail ? "" : *min_element(head->next->keys.begin(), head->next->keys.end()); }
};

int main() {
    AllOne a;
    a.plus("a"); a.plus("a"); a.plus("a"); // a=3
    a.plus("b");                            // b=1
    cout << "get_max: " << a.getMax() << "\n";
    cout << "get_min: " << a.getMin() << "\n";
    a.minus("a"); a.minus("a");             // a=1, b=1
    cout << "get_max: " << a.getMax() << "\n";
    cout << "get_min: " << a.getMin() << "\n";
    return 0;
}
