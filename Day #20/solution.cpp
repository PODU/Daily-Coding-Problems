// Intersection of two linked lists: two-pointer switch trick.
// Time O(M+N), Space O(1).
#include <bits/stdc++.h>
using namespace std;

struct Node {
    int val;
    Node* next;
    Node(int v): val(v), next(nullptr) {}
};

Node* getIntersection(Node* a, Node* b) {
    Node* pa = a;
    Node* pb = b;
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

    Node* a = new Node(3);
    a->next = new Node(7);
    a->next->next = shared;

    Node* b = new Node(99);
    b->next = new Node(1);
    b->next->next = shared;

    Node* res = getIntersection(a, b);
    cout << res->val << "\n";
    return 0;
}
