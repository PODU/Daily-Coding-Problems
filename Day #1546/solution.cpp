// Day 1546: Stable partition of a linked list around pivot k.
// Build two sublists (< k) and (>= k) preserving order, then splice. Time O(n), Space O(1).
#include <bits/stdc++.h>
using namespace std;

struct Node { int val; Node* next; Node(int v): val(v), next(nullptr) {} };

Node* partition(Node* head, int k) {
    Node lessD(0), geD(0);
    Node *l = &lessD, *g = &geD;
    for (Node* c = head; c; c = c->next) {
        if (c->val < k) { l->next = c; l = c; }
        else            { g->next = c; g = c; }
    }
    g->next = nullptr;
    l->next = geD.next;
    return lessD.next;
}

int main() {
    int vals[] = {5, 1, 8, 0, 3};
    Node *head = nullptr, *tail = nullptr;
    for (int v : vals) {
        Node* n = new Node(v);
        if (!head) head = tail = n; else { tail->next = n; tail = n; }
    }
    head = partition(head, 3);
    bool first = true;
    for (Node* c = head; c; c = c->next) {
        if (!first) cout << " -> ";
        cout << c->val; first = false;
    }
    cout << "\n";
}
