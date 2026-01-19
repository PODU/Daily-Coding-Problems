// XOR linked list via raw pointers: each node stores both = addr(prev) XOR addr(next).
// add O(1) with tail tracking, get O(index). O(1) extra per node.
#include <bits/stdc++.h>
using namespace std;

struct Node {
    int value;
    uintptr_t both; // XOR of prev and next addresses
    Node(int v) : value(v), both(0) {}
};

class XorList {
    Node* head = nullptr;
    Node* tail = nullptr;
    vector<Node*> owned; // keep ownership for cleanup

    static uintptr_t ptr(Node* n) { return reinterpret_cast<uintptr_t>(n); }
public:
    ~XorList() { for (auto n : owned) delete n; }

    void add(int element) {
        Node* node = new Node(element);
        owned.push_back(node);
        if (!head) {
            head = tail = node;
        } else {
            tail->both ^= ptr(node);       // old tail: next was null, now node
            node->both = ptr(tail);         // new node: prev = old tail, next = null
            tail = node;
        }
    }

    int get(int index) {
        Node* prev = nullptr;
        Node* cur = head;
        for (int i = 0; i < index && cur; ++i) {
            Node* next = reinterpret_cast<Node*>(cur->both ^ ptr(prev));
            prev = cur;
            cur = next;
        }
        if (!cur) throw out_of_range("index out of range");
        return cur->value;
    }
};

int main() {
    XorList list;
    for (int v : {10, 20, 30, 40, 50}) list.add(v);
    cout << "get(0) = " << list.get(0) << "\n";
    cout << "get(2) = " << list.get(2) << "\n";
    cout << "get(4) = " << list.get(4) << "\n";
    return 0;
}
