// Day 800: Rearrange list values into low->high->low... (wiggle).
// One pass: at even idx ensure a<=next, at odd idx ensure a>=next; swap if not.
// Time O(N), Space O(1).
#include <bits/stdc++.h>
using namespace std;

struct Node { int val; Node* next; Node(int v): val(v), next(nullptr) {} };

void wiggle(Node* head) {
    bool less = true; // even index expects a[i] <= a[i+1]
    for (Node* cur = head; cur && cur->next; cur = cur->next) {
        if (less ? (cur->val > cur->next->val) : (cur->val < cur->next->val))
            swap(cur->val, cur->next->val);
        less = !less;
    }
}

int main() {
    Node* head = new Node(1);
    Node* c = head;
    for (int v : {2, 3, 4, 5}) { c->next = new Node(v); c = c->next; }
    wiggle(head);
    for (Node* p = head; p; p = p->next)
        cout << p->val << (p->next ? " -> " : "\n"); // 1 -> 3 -> 2 -> 5 -> 4
    return 0;
}
