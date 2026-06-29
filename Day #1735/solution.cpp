// Iterative in-place reversal of a singly linked list using three pointers.
// Time: O(n), Space: O(1).
#include <bits/stdc++.h>
using namespace std;

struct Node { int val; Node* next; Node(int v): val(v), next(nullptr) {} };

Node* reverse(Node* head) {
    Node* prev = nullptr;
    while (head) {
        Node* nxt = head->next;
        head->next = prev;
        prev = head;
        head = nxt;
    }
    return prev;
}

int main() {
    Node* head = nullptr;
    for (int i = 5; i >= 1; --i) { Node* n = new Node(i); n->next = head; head = n; }
    head = reverse(head);
    for (Node* c = head; c; c = c->next) cout << c->val << (c->next ? " " : "\n");
    return 0;
}
