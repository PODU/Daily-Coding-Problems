// Day 131: Deep clone a linked list with next + random pointers.
// Interleaving trick (weave copies, set randoms, unweave). O(n) time, O(1) extra space.
#include <bits/stdc++.h>
using namespace std;

struct Node {
    int val;
    Node *next, *random;
    Node(int v) : val(v), next(nullptr), random(nullptr) {}
};

Node* clone(Node* head) {
    if (!head) return nullptr;
    // 1. weave copies
    for (Node* c = head; c; c = c->next->next) {
        Node* cp = new Node(c->val);
        cp->next = c->next;
        c->next = cp;
    }
    // 2. assign randoms
    for (Node* c = head; c; c = c->next->next)
        if (c->random) c->next->random = c->random->next;
    // 3. unweave
    Node* newHead = head->next;
    for (Node* c = head; c; c = c->next) {
        Node* cp = c->next;
        c->next = cp->next;
        if (cp->next) cp->next = cp->next->next;
    }
    return newHead;
}

int main() {
    vector<Node*> n;
    for (int v = 1; v <= 5; v++) n.push_back(new Node(v));
    for (int i = 0; i < 4; i++) n[i]->next = n[i + 1];
    n[0]->random = n[2];
    n[1]->random = n[0];
    n[2]->random = n[4];
    n[3]->random = n[1];
    n[4]->random = n[4];

    Node* copy = clone(n[0]);
    bool separate = true;
    Node* o = n[0];
    Node* c = copy;
    while (c) {
        if (c == o) separate = false;
        cout << "node " << c->val << " -> random " << (c->random ? c->random->val : 0) << endl;
        o = o->next;
        c = c->next;
    }
    cout << "deep copy verified: " << (separate ? "true" : "false") << endl;
    return 0;
}
