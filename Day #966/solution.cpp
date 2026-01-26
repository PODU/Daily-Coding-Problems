// Day 966: Deep clone a linked list where each node has a random pointer.
// Approach: interleave copies, set randoms, split. Time O(n), Space O(1) extra.
#include <bits/stdc++.h>
using namespace std;

struct Node { int val; Node* next; Node* random; Node(int v): val(v), next(nullptr), random(nullptr) {} };

Node* cloneList(Node* head) {
    if (!head) return nullptr;
    for (Node* p = head; p; p = p->next->next) {
        Node* c = new Node(p->val);
        c->next = p->next;
        p->next = c;
    }
    for (Node* p = head; p; p = p->next->next)
        if (p->random) p->next->random = p->random->next;
    Node* newHead = head->next;
    for (Node* p = head; p; p = p->next) {
        Node* c = p->next;
        p->next = c->next;
        if (c->next) c->next = c->next->next;
    }
    return newHead;
}

int main() {
    Node* a = new Node(1); Node* b = new Node(2); Node* c = new Node(3);
    a->next = b; b->next = c;
    a->random = c; b->random = a; c->random = b;

    Node* cl = cloneList(a);
    for (Node* p = cl; p; p = p->next)
        cout << "val=" << p->val << " random=" << (p->random ? p->random->val : -1) << endl;
    // expect: 1->3, 2->1, 3->2
    return 0;
}
