// Day 208: Partition a linked list around pivot k (stable).
// Build two lists (< k and >= k) in original order, then splice. Time: O(n), Space: O(1).
#include <bits/stdc++.h>
using namespace std;

struct Node { int val; Node* next; Node(int v) : val(v), next(nullptr) {} };

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

Node* build(vector<int> v) {
    Node dummy(0), *t = &dummy;
    for (int x : v) { t->next = new Node(x); t = t->next; }
    return dummy.next;
}

void print(Node* h) {
    for (; h; h = h->next) cout << h->val << (h->next ? " -> " : "\n");
}

int main() {
    print(partition(build({5, 1, 8, 0, 3}), 3)); // 1 -> 0 -> 5 -> 8 -> 3
    return 0;
}
