// Day 1143: Merge k sorted linked lists.
// Min-heap of list heads. Time O(N log k), Space O(k).
#include <bits/stdc++.h>
using namespace std;

struct Node { int val; Node* next; Node(int v): val(v), next(nullptr) {} };

Node* mergeK(vector<Node*>& lists) {
    auto cmp = [](Node* a, Node* b) { return a->val > b->val; };
    priority_queue<Node*, vector<Node*>, decltype(cmp)> pq(cmp);
    for (Node* l : lists) if (l) pq.push(l);
    Node dummy(0), *tail = &dummy;
    while (!pq.empty()) {
        Node* n = pq.top(); pq.pop();
        tail->next = n; tail = n;
        if (n->next) pq.push(n->next);
    }
    return dummy.next;
}

Node* build(vector<int> v) {
    Node dummy(0), *t = &dummy;
    for (int x : v) { t->next = new Node(x); t = t->next; }
    return dummy.next;
}

int main() {
    vector<Node*> lists = { build({1, 4, 7}), build({2, 5, 8}), build({3, 6, 9}) };
    for (Node* n = mergeK(lists); n; n = n->next) cout << n->val << (n->next ? " -> " : "\n");
    return 0; // 1 -> 2 -> 3 -> 4 -> 5 -> 6 -> 7 -> 8 -> 9
}
