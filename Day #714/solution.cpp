// Day 714: Swap every two adjacent nodes in a singly linked list. Iterative
// pointer rewiring with a dummy head. Time O(n), space O(1).
#include <bits/stdc++.h>
using namespace std;

struct Node { int val; Node* next; Node(int v): val(v), next(nullptr) {} };

Node* swapPairs(Node* head) {
    Node dummy(0); dummy.next = head;
    Node* prev = &dummy;
    while (prev->next && prev->next->next) {
        Node* a = prev->next;
        Node* b = a->next;
        a->next = b->next;
        b->next = a;
        prev->next = b;
        prev = a;
    }
    return dummy.next;
}

int main() {
    Node* head = new Node(1);
    head->next = new Node(2);
    head->next->next = new Node(3);
    head->next->next->next = new Node(4);
    head = swapPairs(head);
    for (Node* c = head; c; c = c->next)
        cout << c->val << (c->next ? " -> " : "\n");
    return 0;
}
