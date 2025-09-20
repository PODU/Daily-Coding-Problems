// Day 305: Remove consecutive nodes summing to zero. Prefix-sum + hashmap. O(N).
#include <bits/stdc++.h>
using namespace std;
struct Node { int val; Node* next; Node(int v) : val(v), next(nullptr) {} };
Node* removeZeroSum(Node* head) {
    Node dummy(0); dummy.next = head;
    unordered_map<int, Node*> seen;
    int prefix = 0;
    for (Node* cur = &dummy; cur; cur = cur->next) { prefix += cur->val; seen[prefix] = cur; } // last occurrence
    prefix = 0;
    for (Node* cur = &dummy; cur; cur = cur->next) { prefix += cur->val; cur->next = seen[prefix]->next; }
    return dummy.next;
}
int main() {
    int vals[] = {3, 4, -7, 5, -6, 6};
    Node *head = nullptr, *tail = nullptr;
    for (int v : vals) { Node* n = new Node(v); if (!head) head = tail = n; else { tail->next = n; tail = n; } }
    head = removeZeroSum(head);
    vector<int> out;
    for (Node* c = head; c; c = c->next) out.push_back(c->val);
    for (size_t i = 0; i < out.size(); i++) cout << out[i] << (i + 1 < out.size() ? " " : "");
    cout << "\n"; // 5
}
