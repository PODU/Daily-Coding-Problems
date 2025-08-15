// Day 127: Add two numbers stored as reversed-digit linked lists.
// Single pass with carry. O(max(m,n)) time, O(1) extra space.
#include <bits/stdc++.h>
using namespace std;

struct Node {
    int val;
    Node* next;
    Node(int v) : val(v), next(nullptr) {}
};

Node* addLists(Node* a, Node* b) {
    Node dummy(0);
    Node* tail = &dummy;
    int carry = 0;
    while (a || b || carry) {
        int s = carry;
        if (a) { s += a->val; a = a->next; }
        if (b) { s += b->val; b = b->next; }
        carry = s / 10;
        tail->next = new Node(s % 10);
        tail = tail->next;
    }
    return dummy.next;
}

Node* build(vector<int> d) {
    Node dummy(0);
    Node* t = &dummy;
    for (int v : d) { t->next = new Node(v); t = t->next; }
    return dummy.next;
}

void print(Node* n) {
    while (n) { cout << n->val; if (n->next) cout << " -> "; n = n->next; }
    cout << endl;
}

int main() {
    Node* a = build({9, 9}); // 99
    Node* b = build({5, 2}); // 25
    print(addLists(a, b));   // 124 -> 4 -> 2 -> 1
    return 0;
}
