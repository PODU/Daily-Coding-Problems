// Bottom-up (iterative) merge sort on a singly linked list. O(n log n) time, O(1) space.
#include <iostream>
using namespace std;

struct Node { int val; Node* next; Node(int v): val(v), next(nullptr) {} };

Node* merge(Node* a, Node* b, Node** tailOut) {
    Node dummy(0); Node* tail = &dummy;
    while (a && b) {
        if (a->val <= b->val) { tail->next = a; a = a->next; }
        else { tail->next = b; b = b->next; }
        tail = tail->next;
    }
    tail->next = a ? a : b;
    while (tail->next) tail = tail->next;
    *tailOut = tail;
    return dummy.next;
}

int length(Node* h) { int n = 0; while (h) { n++; h = h->next; } return n; }

Node* split(Node* head, int n) {
    for (int i = 1; head && i < n; i++) head = head->next;
    if (!head) return nullptr;
    Node* rest = head->next; head->next = nullptr; return rest;
}

Node* sortList(Node* head) {
    int n = length(head);
    Node dummy(0); dummy.next = head;
    for (int size = 1; size < n; size <<= 1) {
        Node* cur = dummy.next; Node* tail = &dummy;
        while (cur) {
            Node* left = cur;
            Node* right = split(left, size);
            cur = split(right, size);
            Node* mergedTail;
            tail->next = merge(left, right, &mergedTail);
            tail = mergedTail;
        }
    }
    return dummy.next;
}

int main() {
    int vals[] = {4, 1, -3, 99};
    Node dummy(0); Node* t = &dummy;
    for (int v : vals) { t->next = new Node(v); t = t->next; }
    Node* head = sortList(dummy.next);
    bool first = true;
    for (Node* p = head; p; p = p->next) { if (!first) cout << " -> "; cout << p->val; first = false; }
    cout << endl;
    return 0;
}
