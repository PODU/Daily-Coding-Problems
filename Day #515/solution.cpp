// Partition list: build "less than k" and ">= k" sublists, then join. O(n) time, O(1) extra.
#include <bits/stdc++.h>
using namespace std;

struct Node { int val; Node* next; Node(int v): val(v), next(nullptr) {} };

Node* partition(Node* head, int k) {
    Node lessDummy(0), geDummy(0);
    Node *less = &lessDummy, *ge = &geDummy;
    for (Node* cur = head; cur; cur = cur->next) {
        if (cur->val < k) { less->next = cur; less = cur; }
        else { ge->next = cur; ge = cur; }
    }
    ge->next = nullptr;
    less->next = geDummy.next;
    return lessDummy.next;
}

int main() {
    int vals[] = {5, 1, 8, 0, 3};
    Node* head = nullptr; Node* tail = nullptr;
    for (int v : vals) {
        Node* n = new Node(v);
        if (!head) head = tail = n; else { tail->next = n; tail = n; }
    }
    head = partition(head, 3);
    for (Node* c = head; c; c = c->next) cout << c->val << (c->next ? " -> " : "\n");
    return 0;
}
