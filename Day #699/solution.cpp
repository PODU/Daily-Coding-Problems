// Day 699: Rotate a singly linked list right by k places.
// Approach: close into a ring, then break it (len - k%len) nodes ahead.
// Time O(n), Space O(1).
#include <bits/stdc++.h>
using namespace std;

struct Node { int val; Node* next; Node(int v) : val(v), next(nullptr) {} };

Node* rotateRight(Node* head, int k) {
    if (!head || !head->next) return head;
    int len = 1; Node* tail = head;
    while (tail->next) { tail = tail->next; ++len; }
    k %= len; if (k == 0) return head;
    tail->next = head;                 // make ring
    int stepsToNewTail = len - k;
    Node* newTail = head;
    for (int i = 1; i < stepsToNewTail; ++i) newTail = newTail->next;
    Node* newHead = newTail->next;
    newTail->next = nullptr;
    return newHead;
}

Node* build(vector<int> v) { Node d(0); Node* c = &d; for (int x : v) { c->next = new Node(x); c = c->next; } return d.next; }
void print(Node* h) { for (; h; h = h->next) cout << h->val << (h->next ? " -> " : "\n"); }

int main() {
    print(rotateRight(build({7, 7, 3, 5}), 2));       // 3 -> 5 -> 7 -> 7
    print(rotateRight(build({1, 2, 3, 4, 5}), 3));    // 3 -> 4 -> 5 -> 1 -> 2
    return 0;
}
