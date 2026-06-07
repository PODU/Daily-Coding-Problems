// Day 1626: Add two numbers stored as reversed-digit linked lists.
// Single pass with carry. Time O(max(m,n)), Space O(max(m,n)).
#include <bits/stdc++.h>
using namespace std;

struct Node { int val; Node* next = nullptr; Node(int v) : val(v) {} };

Node* addLists(Node* a, Node* b) {
    Node dummy(0); Node* tail = &dummy; int carry = 0;
    while (a || b || carry) {
        int sum = carry;
        if (a) { sum += a->val; a = a->next; }
        if (b) { sum += b->val; b = b->next; }
        carry = sum / 10;
        tail->next = new Node(sum % 10);
        tail = tail->next;
    }
    return dummy.next;
}

Node* build(vector<int> v) {
    Node dummy(0); Node* t = &dummy;
    for (int x : v) { t->next = new Node(x); t = t->next; }
    return dummy.next;
}

int main() {
    Node* r = addLists(build({9, 9}), build({5, 2}));
    for (Node* c = r; c; c = c->next) cout << c->val << (c->next ? " -> " : "\n");
    return 0;
}
