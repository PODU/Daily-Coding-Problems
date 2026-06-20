// Two-pointer intersection of singly linked lists: redirect each pointer to the other head at end.
// Time O(M+N), Space O(1).
#include <iostream>
using namespace std;

struct Node {
    int val;
    Node* next;
    Node(int v): val(v), next(nullptr) {}
};

Node* getIntersection(Node* headA, Node* headB) {
    if (!headA || !headB) return nullptr;
    Node* pA = headA;
    Node* pB = headB;
    while (pA != pB) {
        pA = pA ? pA->next : headB;
        pB = pB ? pB->next : headA;
    }
    return pA;
}

int main() {
    // Shared tail: 8 -> 10
    Node* n8 = new Node(8);
    n8->next = new Node(10);
    // A = 3 -> 7 -> 8 -> 10
    Node* a = new Node(3);
    a->next = new Node(7);
    a->next->next = n8;
    // B = 99 -> 1 -> 8 -> 10 (same node 8)
    Node* b = new Node(99);
    b->next = new Node(1);
    b->next->next = n8;

    Node* res = getIntersection(a, b);
    cout << (res ? to_string(res->val) : string("null")) << endl;
    return 0;
}
