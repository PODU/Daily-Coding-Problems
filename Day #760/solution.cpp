// Day 760: Uniformly shuffle a linked list. Space-prioritized variant:
// forward Fisher-Yates that swaps node values in place using O(1) extra space
// at the cost of O(n^2) time (re-walks to pick a uniform remaining node).
// A deterministic LCG is used so the demo output is reproducible.
#include <bits/stdc++.h>
using namespace std;

struct Node { int val; Node* next; Node(int v): val(v), next(nullptr) {} };

struct LCG {
    unsigned long long s;
    LCG(unsigned long long seed): s(seed) {}
    unsigned int next() { s = (s * 1103515245ULL + 12345ULL) & 0x7fffffffULL; return (unsigned)s; }
};

void shuffle(Node* head, LCG& rng) {
    for (Node* p = head; p; p = p->next) {
        int m = 0;                       // count remaining nodes from p
        for (Node* t = p; t; t = t->next) ++m;
        int r = rng.next() % m;          // pick uniform offset
        Node* q = p;
        while (r--) q = q->next;
        swap(p->val, q->val);            // O(1) space swap
    }
}

void printList(Node* head) {
    for (Node* p = head; p; p = p->next)
        cout << p->val << (p->next ? " -> " : "\n");
}

int main() {
    Node* head = new Node(1);
    Node* cur = head;
    for (int v = 2; v <= 5; ++v) { cur->next = new Node(v); cur = cur->next; }

    cout << "original: "; printList(head);
    LCG rng(42);
    shuffle(head, rng);
    cout << "shuffled: "; printList(head);
    return 0;
}
