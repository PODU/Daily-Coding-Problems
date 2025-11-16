// Day 611: All O(1) structure (plus / minus / get_max / get_min).
// Approach: doubly-linked list of value-buckets (set of keys) + key->bucket map. All ops O(1).
#include <bits/stdc++.h>
using namespace std;

class AllOne {
    struct Bucket {
        int value;
        unordered_set<string> keys;
        Bucket *prev = nullptr, *next = nullptr;
        Bucket(int v) : value(v) {}
    };
    Bucket *head, *tail; // sentinels; buckets kept in ascending value order
    unordered_map<string, Bucket*> keyToBucket;

    Bucket* insertAfter(Bucket* node, int value) {
        Bucket* b = new Bucket(value);
        b->prev = node; b->next = node->next;
        node->next->prev = b; node->next = b;
        return b;
    }
    void removeBucket(Bucket* node) {
        node->prev->next = node->next;
        node->next->prev = node->prev;
        delete node;
    }
public:
    AllOne() {
        head = new Bucket(0); tail = new Bucket(0);
        head->next = tail; tail->prev = head;
    }
    void plus(const string& key) {
        if (keyToBucket.count(key)) {
            Bucket* cur = keyToBucket[key];
            Bucket* nxt = cur->next;
            if (nxt == tail || nxt->value != cur->value + 1)
                nxt = insertAfter(cur, cur->value + 1);
            nxt->keys.insert(key);
            keyToBucket[key] = nxt;
            cur->keys.erase(key);
            if (cur->keys.empty()) removeBucket(cur);
        } else {
            Bucket* first = head->next;
            if (first == tail || first->value != 1)
                first = insertAfter(head, 1);
            first->keys.insert(key);
            keyToBucket[key] = first;
        }
    }
    void minus(const string& key) {
        if (!keyToBucket.count(key)) return;
        Bucket* cur = keyToBucket[key];
        if (cur->value == 1) {
            keyToBucket.erase(key);
        } else {
            Bucket* prv = cur->prev;
            if (prv == head || prv->value != cur->value - 1)
                prv = insertAfter(cur->prev, cur->value - 1);
            prv->keys.insert(key);
            keyToBucket[key] = prv;
        }
        cur->keys.erase(key);
        if (cur->keys.empty()) removeBucket(cur);
    }
    string get_max() { return tail->prev == head ? "" : *min_element(tail->prev->keys.begin(), tail->prev->keys.end()); }
    string get_min() { return head->next == tail ? "" : *min_element(head->next->keys.begin(), head->next->keys.end()); }
};

int main() {
    AllOne a;
    a.plus("a"); a.plus("b"); a.plus("a"); // a=2, b=1
    cout << a.get_max() << "\n"; // a
    cout << a.get_min() << "\n"; // b
    return 0;
}
