// Rearrange linked list values to low->high->low->high. One pass swapping
// adjacent values to enforce the alternating relation. Time O(n), Space O(1).
#include <bits/stdc++.h>
using namespace std;

struct Node { int val; Node* next; Node(int v):val(v),next(nullptr){} };

void zigzag(Node* head) {
    bool low = true; // current pair should satisfy a <= b
    for (Node* c = head; c && c->next; c = c->next, low = !low) {
        if (low ? (c->val > c->next->val) : (c->val < c->next->val))
            swap(c->val, c->next->val);
    }
}

int main() {
    Node* head = nullptr; Node** t = &head;
    for (int v : {1,2,3,4,5}) { *t = new Node(v); t = &(*t)->next; }
    zigzag(head);
    for (Node* c = head; c; c = c->next)
        cout << c->val << (c->next ? " -> " : "\n"); // 1 -> 3 -> 2 -> 5 -> 4
    return 0;
}
