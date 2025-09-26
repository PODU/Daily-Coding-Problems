// Shuffle linked list uniformly via Fisher-Yates on node values.
// O(n) time, O(1) extra (in-place value swaps). Fixed seed -> deterministic.
#include <bits/stdc++.h>
using namespace std;

struct Node { int val; Node* next; Node(int v):val(v),next(nullptr){} };

int main() {
    // build 1->2->3->4->5
    Node* head = nullptr; Node* tail = nullptr;
    for (int v = 1; v <= 5; ++v) {
        Node* n = new Node(v);
        if (!head) head = tail = n; else { tail->next = n; tail = n; }
    }
    // gather node pointers
    vector<Node*> nodes;
    for (Node* p = head; p; p = p->next) nodes.push_back(p);
    int n = (int)nodes.size();

    mt19937 rng(42);
    // Fisher-Yates: swap values in place
    for (int i = n - 1; i > 0; --i) {
        int j = (int)(rng() % (i + 1));
        swap(nodes[i]->val, nodes[j]->val);
    }

    bool first = true;
    for (Node* p = head; p; p = p->next) {
        if (!first) cout << ' ';
        cout << p->val; first = false;
    }
    cout << '\n';
    return 0;
}
