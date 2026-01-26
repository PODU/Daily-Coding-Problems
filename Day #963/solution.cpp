// Day 963: Find intersecting node of two singly linked lists.
// Approach: two pointers swap heads; meet at intersection. Time O(M+N), Space O(1).
#include <bits/stdc++.h>
using namespace std;

struct Node { int val; Node* next; Node(int v): val(v), next(nullptr) {} };

Node* getIntersection(Node* a, Node* b) {
    if (!a || !b) return nullptr;
    Node *p = a, *q = b;
    while (p != q) {
        p = p ? p->next : b;
        q = q ? q->next : a;
    }
    return p;
}

int main() {
    // A = 3 -> 7 -> 8 -> 10 ; B = 99 -> 1 -> 8 -> 10 (shared from node 8)
    Node* n8 = new Node(8);
    n8->next = new Node(10);
    Node* a = new Node(3); a->next = new Node(7); a->next->next = n8;
    Node* b = new Node(99); b->next = new Node(1); b->next->next = n8;

    Node* res = getIntersection(a, b);
    cout << "the node with value " << res->val << endl;
    return 0;
}
