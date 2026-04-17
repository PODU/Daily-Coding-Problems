// Uniform linked-list shuffle via Fisher-Yates. Time O(n), Space O(n) for the
// index pass (space-over-time variant: O(1) space, O(n^2) by random node selection).
// A deterministic LCG is used so output is reproducible.
#include <bits/stdc++.h>
using namespace std;

struct Node { int val; Node* next; Node(int v): val(v), next(nullptr) {} };

unsigned long long seed = 42;
unsigned long long nextRand() {
    seed = (seed * 1103515245ULL + 12345ULL) % 2147483648ULL;
    return seed;
}

Node* shuffle(Node* head) {
    vector<Node*> nodes;
    for (Node* p = head; p; p = p->next) nodes.push_back(p);
    int n = nodes.size();
    for (int i = n - 1; i >= 1; i--) {
        int j = (int)(nextRand() % (i + 1));
        swap(nodes[i]->val, nodes[j]->val);
    }
    return head;
}

int main() {
    Node* head = nullptr; Node* tail = nullptr;
    for (int v = 1; v <= 5; v++) {
        Node* node = new Node(v);
        if (!head) head = tail = node; else { tail->next = node; tail = node; }
    }
    head = shuffle(head);
    bool first = true;
    for (Node* p = head; p; p = p->next) { cout << (first ? "" : " -> ") << p->val; first = false; }
    cout << "\n";
    return 0;
}
