// LRU cache via hash map + doubly linked list; get/set O(1), evict LRU on overflow. O(1) time, O(n) space.
#include <iostream>
#include <unordered_map>

class LRUCache {
    struct Node {
        int key, val;
        Node* prev;
        Node* next;
        Node(int k, int v) : key(k), val(v), prev(nullptr), next(nullptr) {}
    };
    int cap;
    std::unordered_map<int, Node*> map;
    Node* head; // sentinel: most recent side
    Node* tail; // sentinel: least recent side

    void remove(Node* n) {
        n->prev->next = n->next;
        n->next->prev = n->prev;
    }
    void addFront(Node* n) {
        n->next = head->next;
        n->prev = head;
        head->next->prev = n;
        head->next = n;
    }

public:
    LRUCache(int capacity) : cap(capacity) {
        head = new Node(0, 0);
        tail = new Node(0, 0);
        head->next = tail;
        tail->prev = head;
    }

    // returns true if found; value via out param
    bool get(int key, int& out) {
        auto it = map.find(key);
        if (it == map.end()) return false;
        Node* n = it->second;
        remove(n);
        addFront(n);
        out = n->val;
        return true;
    }

    void set(int key, int value) {
        auto it = map.find(key);
        if (it != map.end()) {
            it->second->val = value;
            remove(it->second);
            addFront(it->second);
            return;
        }
        if ((int)map.size() == cap) {
            Node* lru = tail->prev;
            remove(lru);
            map.erase(lru->key);
            delete lru;
        }
        Node* n = new Node(key, value);
        addFront(n);
        map[key] = n;
    }
};

int main() {
    LRUCache cache(2);
    auto printGet = [&](int key) {
        int out;
        if (cache.get(key, out)) std::cout << out << "\n";
        else std::cout << "null\n";
    };

    cache.set(1, 1);
    cache.set(2, 2);
    printGet(1);      // 1
    cache.set(3, 3);  // evicts key 2
    printGet(2);      // null
    cache.set(4, 4);  // evicts key 1
    printGet(1);      // null
    printGet(3);      // 3
    printGet(4);      // 4
    return 0;
}
