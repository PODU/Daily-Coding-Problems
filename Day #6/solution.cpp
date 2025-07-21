// XOR linked list using real pointers; each node stores both = prev XOR next.
// add: O(1), get(i): O(i). Space: O(n).
#include <bits/stdc++.h>
using namespace std;

struct Node {
    int val;
    uintptr_t both; // (uintptr_t)prev XOR (uintptr_t)next
    Node(int v) : val(v), both(0) {}
};

struct XorList {
    Node* head = nullptr;
    Node* tail = nullptr;

    void add(int val) {
        Node* node = new Node(val);
        if (!head) { head = tail = node; return; }
        tail->both ^= (uintptr_t)node;        // set tail.next = node
        node->both = (uintptr_t)tail;          // node.prev = tail, next = null
        tail = node;
    }

    Node* get(int index) {
        uintptr_t prev = 0;
        Node* cur = head;
        for (int i = 0; i < index && cur; i++) {
            uintptr_t next = cur->both ^ prev;
            prev = (uintptr_t)cur;
            cur = (Node*)next;
        }
        return cur;
    }
};

int main() {
    XorList l;
    for (int v : {10, 20, 30, 40}) l.add(v);
    for (int i = 0; i < 4; i++) cout << l.get(i)->val << (i < 3 ? " " : "\n");
    return 0;
}
