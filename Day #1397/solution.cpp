// Two-pointer: advance pa/pb; on reaching end switch to other head.
// They meet at intersection after at most M+N steps. Time O(M+N), Space O(1).
#include <bits/stdc++.h>
using namespace std;

struct Node { int val; Node* next; Node(int v):val(v),next(nullptr){} };

Node* getIntersection(Node* a, Node* b) {
    if (!a || !b) return nullptr;
    Node *pa = a, *pb = b;
    while (pa != pb) {
        pa = pa ? pa->next : b;
        pb = pb ? pb->next : a;
    }
    return pa;
}

int main() {
    // shared tail 8 -> 10
    Node* shared = new Node(8);
    shared->next = new Node(10);
    Node* a = new Node(3); a->next = new Node(7); a->next->next = shared;
    Node* b = new Node(99); b->next = new Node(1); b->next->next = shared;
    Node* r = getIntersection(a, b);
    cout << "the node with value " << r->val << "\n";
    return 0;
}
