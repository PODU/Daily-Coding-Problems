// Day 1205: Add two numbers stored as reversed-digit linked lists.
// Traverse both lists with a running carry. Time O(max(m,n)), Space O(max(m,n)).
#include <bits/stdc++.h>
using namespace std;

struct Node { int val; Node* next; Node(int v): val(v), next(nullptr) {} };

Node* build(vector<int> ds) {
    Node dummy(0), *t = &dummy;
    for (int d : ds) { t->next = new Node(d); t = t->next; }
    return dummy.next;
}

Node* addLists(Node* a, Node* b) {
    Node dummy(0), *t = &dummy; int carry = 0;
    while (a || b || carry) {
        int s = carry + (a ? a->val : 0) + (b ? b->val : 0);
        carry = s / 10;
        t->next = new Node(s % 10); t = t->next;
        if (a) a = a->next;
        if (b) b = b->next;
    }
    return dummy.next;
}

int main() {
    Node* a = build({9, 9}); // 99
    Node* b = build({5, 2}); // 25
    Node* s = addLists(a, b);
    string out;
    for (Node* p = s; p; p = p->next) { if (!out.empty()) out += " -> "; out += to_string(p->val); }
    cout << out << "\n"; // 4 -> 2 -> 1
    return 0;
}
