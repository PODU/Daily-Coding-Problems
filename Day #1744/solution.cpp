// Merge k sorted linked lists via min-heap of current heads. O(N log k) time, O(k) space.
#include <bits/stdc++.h>
using namespace std;

struct Node {
    int val;
    Node* next;
    Node(int v) : val(v), next(nullptr) {}
};

Node* build(vector<int> v) {
    Node dummy(0), *t = &dummy;
    for (int x : v) { t->next = new Node(x); t = t->next; }
    return dummy.next;
}

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

int main() {
    vector<Node*> lists = { build({1,4,5}), build({1,3,4}), build({2,6}) };
    Node* m = mergeK(lists);
    bool first = true;
    for (Node* p = m; p; p = p->next) { if (!first) cout << " "; cout << p->val; first = false; }
    cout << "\n";
    return 0;
}
