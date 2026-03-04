// Day 1148: Rotate linked list right by k.
// Find length, close into ring, cut at (len - k%len). O(n) time, O(1) space.
#include <bits/stdc++.h>
using namespace std;

struct Node { int val; Node* next; Node(int v): val(v), next(nullptr) {} };

Node* rotate(Node* head, int k) {
    if (!head || !head->next) return head;
    int len = 1;
    Node* tail = head;
    while (tail->next) { tail = tail->next; len++; }
    k %= len;
    if (k == 0) return head;
    tail->next = head;                 // close ring
    int stepsToNewTail = len - k;
    Node* newTail = head;
    for (int i = 1; i < stepsToNewTail; ++i) newTail = newTail->next;
    Node* newHead = newTail->next;
    newTail->next = nullptr;
    return newHead;
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
    print(rotate(build({7, 7, 3, 5}), 2));       // 3 -> 5 -> 7 -> 7
    print(rotate(build({1, 2, 3, 4, 5}), 3));     // 3 -> 4 -> 5 -> 1 -> 2
    return 0;
}
