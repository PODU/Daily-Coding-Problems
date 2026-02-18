// Uniformly shuffle a linked list via Fisher-Yates on node values. Time O(n), Space O(n).
// Space-over-time alternative: walk to a random remaining node and swap in place -> O(1) extra, O(n^2) time.
#include <bits/stdc++.h>
using namespace std;

struct Node { int val; Node* next; Node(int v) : val(v), next(nullptr) {} };

Node* build(vector<int> v) {
    Node *head = nullptr, *tail = nullptr;
    for (int x : v) { Node* n = new Node(x); if (!head) head = tail = n; else { tail->next = n; tail = n; } }
    return head;
}
void print(Node* h) {
    cout << "["; bool f = true;
    for (; h; h = h->next) { if (!f) cout << ", "; cout << h->val; f = false; }
    cout << "]\n";
}
void shuffleList(Node* head, mt19937& rng) {
    vector<Node*> nodes;
    for (Node* c = head; c; c = c->next) nodes.push_back(c);
    for (int i = (int)nodes.size() - 1; i > 0; i--) {
        int j = rng() % (i + 1);
        swap(nodes[i]->val, nodes[j]->val);
    }
}

int main() {
    Node* head = build({1, 2, 3, 4, 5});
    cout << "Original: "; print(head);
    mt19937 rng(42);
    shuffleList(head, rng);
    cout << "Shuffled: "; print(head);
}
