// Day 452: Add two numbers stored as reversed-digit linked lists.
// Single pass with carry. Time O(max(n,m)), Space O(max(n,m)).
#include <iostream>
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
        int sum = carry;
        if (a) { sum += a->val; a = a->next; }
        if (b) { sum += b->val; b = b->next; }
        carry = sum / 10;
        tail->next = new Node(sum % 10);
        tail = tail->next;
    }
    return dummy.next;
}

Node* build(initializer_list<int> xs) {
    Node dummy(0);
    Node* t = &dummy;
    for (int x : xs) { t->next = new Node(x); t = t->next; }
    return dummy.next;
}

void print(Node* n) {
    bool first = true;
    while (n) { cout << (first ? "" : " -> ") << n->val; first = false; n = n->next; }
    cout << endl;
}

int main() {
    Node* a = build({9, 9}); // 99
    Node* b = build({5, 2}); // 25
    print(addLists(a, b));   // 4 -> 2 -> 1
    return 0;
}
