// Deep clone list w/ random ptr: interleave clones, wire randoms, unweave. O(n) time, O(1) extra.
#include <bits/stdc++.h>
using namespace std;

struct Node {
    int val;
    Node* next;
    Node* random;
    Node(int v) : val(v), next(nullptr), random(nullptr) {}
};

Node* copyRandomList(Node* head) {
    if (!head) return nullptr;
    for (Node* c = head; c; c = c->next->next) {
        Node* cl = new Node(c->val);
        cl->next = c->next;
        c->next = cl;
    }
    for (Node* c = head; c; c = c->next->next)
        c->next->random = c->random ? c->random->next : nullptr;
    Node* newHead = head->next;
    for (Node* c = head; c; c = c->next) {
        Node* cl = c->next;
        c->next = cl->next;
        cl->next = cl->next ? cl->next->next : nullptr;
    }
    return newHead;
}

int main() {
    Node* n1 = new Node(1);
    Node* n2 = new Node(2);
    Node* n3 = new Node(3);
    Node* n4 = new Node(4);
    n1->next = n2; n2->next = n3; n3->next = n4;
    n1->random = n3;
    n2->random = n1;
    n3->random = n3;
    n4->random = n2;

    Node* cloned = copyRandomList(n1);
    for (Node* c = cloned; c; c = c->next)
        cout << "node " << c->val << ", random " << c->random->val << "\n";
    return 0;
}
