// Day 1486: Partition a linked list around pivot k (stable).
// Approach: build two sublists (< k and >= k), then concatenate. O(n) time, O(1) extra space.
#include <iostream>
using namespace std;

struct Node { int val; Node* next; Node(int v): val(v), next(nullptr) {} };

Node* partition(Node* head, int k) {
    Node lessDummy(0), geDummy(0);
    Node* less = &lessDummy;
    Node* ge = &geDummy;
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
    for (Node* c = head; c; c = c->next) {
        cout << c->val;
        if (c->next) cout << " -> ";
    }
    cout << endl;
    return 0;
}
