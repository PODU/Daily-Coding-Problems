// Intersection: two pointers switch lists after end; meet at the join. O(M+N) time, O(1) space.
#include <bits/stdc++.h>
using namespace std;

struct Node { int val; Node* next; Node(int v): val(v), next(nullptr) {} };

Node* getIntersection(Node* a, Node* b) {
    if (!a || !b) return nullptr;
    Node *p = a, *q = b;
    while (p != q) {
        p = p ? p->next : b;
        q = q ? q->next : a;
    }
    return p;
}

int main() {
    Node* shared = new Node(8);
    shared->next = new Node(10);
    Node* A = new Node(3); A->next = new Node(7); A->next->next = shared;
    Node* B = new Node(99); B->next = new Node(1); B->next->next = shared;
    Node* r = getIntersection(A, B);
    cout << "The node with value " << r->val << "\n";
    return 0;
}
