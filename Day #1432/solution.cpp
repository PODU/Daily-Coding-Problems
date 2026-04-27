// Day 1432: Deep clone a linked list with a random pointer.
// Approach: interleave cloned nodes, wire randoms, then split. Time: O(n), Space: O(1) extra.
#include <bits/stdc++.h>
using namespace std;

struct Node {
    int val;
    Node* next;
    Node* random;
    Node(int v) : val(v), next(nullptr), random(nullptr) {}
};

Node* cloneList(Node* head) {
    if (!head) return nullptr;
    // 1. interleave: A -> A' -> B -> B' ...
    for (Node* cur = head; cur; cur = cur->next->next) {
        Node* copy = new Node(cur->val);
        copy->next = cur->next;
        cur->next = copy;
    }
    // 2. set random pointers on copies
    for (Node* cur = head; cur; cur = cur->next->next) {
        if (cur->random) cur->next->random = cur->random->next;
    }
    // 3. split the two lists
    Node* newHead = head->next;
    for (Node* cur = head; cur; cur = cur->next) {
        Node* copy = cur->next;
        cur->next = copy->next;
        if (copy->next) copy->next = copy->next->next;
    }
    return newHead;
}

int main() {
    // Build 1 -> 2 -> 3; randoms: 1->3, 2->1, 3->3
    Node* a = new Node(1);
    Node* b = new Node(2);
    Node* c = new Node(3);
    a->next = b; b->next = c;
    a->random = c; b->random = a; c->random = c;

    Node* cloned = cloneList(a);
    bool ok = true;
    Node* p = a; Node* q = cloned;
    while (p) {
        if (q == p) ok = false;                 // must be distinct nodes
        if (q->val != p->val) ok = false;       // same values
        if ((p->random == nullptr) != (q->random == nullptr)) ok = false;
        if (p->random && q->random->val != p->random->val) ok = false;
        p = p->next; q = q->next;
    }
    cout << (ok ? "Clone verified: values and random targets match, nodes distinct"
                : "Clone FAILED") << endl;
    return 0;
}
