// Sort a linked list in O(n log n) time, O(1) extra space.
// Bottom-up (iterative) merge sort on the list; no recursion stack -> constant space.
#include <bits/stdc++.h>
using namespace std;

struct Node {
    int val;
    Node* next;
    Node(int v) : val(v), next(nullptr) {}
};

Node* split(Node* head, int size) {
    for (int i = 1; head && i < size; ++i) head = head->next;
    if (!head) return nullptr;
    Node* rest = head->next;
    head->next = nullptr;
    return rest;
}

Node* merge(Node* a, Node* b, Node* tail) {
    while (a && b) {
        if (a->val <= b->val) { tail->next = a; a = a->next; }
        else { tail->next = b; b = b->next; }
        tail = tail->next;
    }
    tail->next = a ? a : b;
    while (tail->next) tail = tail->next;
    return tail;
}

Node* sortList(Node* head) {
    if (!head || !head->next) return head;
    int n = 0;
    for (Node* p = head; p; p = p->next) ++n;
    Node dummy(0);
    dummy.next = head;
    for (int size = 1; size < n; size *= 2) {
        Node* cur = dummy.next;
        Node* tail = &dummy;
        while (cur) {
            Node* left = cur;
            Node* right = split(left, size);
            cur = split(right, size);
            tail = merge(left, right, tail);
        }
    }
    return dummy.next;
}

int main() {
    int vals[] = {4, 1, -3, 99};
    Node dummy(0);
    Node* t = &dummy;
    for (int v : vals) { t->next = new Node(v); t = t->next; }
    Node* head = sortList(dummy.next);
    bool first = true;
    for (Node* p = head; p; p = p->next) {
        if (!first) cout << " -> ";
        cout << p->val;
        first = false;
    }
    cout << endl;
    return 0;
}
