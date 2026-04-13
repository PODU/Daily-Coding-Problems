// Remove consecutive nodes summing to zero: dummy head, prefix-sum -> last node map;
// repeated prefix means a zero-sum span to splice out. Time O(n), Space O(n).
#include <bits/stdc++.h>
using namespace std;

struct Node { int val; Node* next; Node(int v):val(v),next(nullptr){} };

Node* removeZeroSum(Node* head) {
    Node* dummy = new Node(0);
    dummy->next = head;
    unordered_map<int, Node*> seen;
    int prefix = 0;
    for (Node* cur = dummy; cur; cur = cur->next) {
        prefix += cur->val;
        seen[prefix] = cur; // last node achieving this prefix sum
    }
    prefix = 0;
    for (Node* cur = dummy; cur; cur = cur->next) {
        prefix += cur->val;
        cur->next = seen[prefix]->next; // skip zero-sum span
    }
    return dummy->next;
}

int main() {
    int vals[] = {3, 4, -7, 5, -6, 6};
    Node* head = nullptr; Node* tail = nullptr;
    for (int v : vals) {
        Node* n = new Node(v);
        if (!head) head = tail = n; else { tail->next = n; tail = n; }
    }
    head = removeZeroSum(head);
    string out;
    for (Node* c = head; c; c = c->next) {
        if (!out.empty()) out += " -> ";
        out += to_string(c->val);
    }
    cout << out << "\n";
    return 0;
}
