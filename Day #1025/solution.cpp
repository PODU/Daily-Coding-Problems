// Day 1025: Remove all consecutive linked-list nodes that sum to zero.
// Approach: prefix-sum + hashmap of last node per prefix sum (dummy head). O(N) time, O(N) space.
#include <bits/stdc++.h>
using namespace std;

struct Node {
    int val;
    Node* next;
    Node(int v) : val(v), next(nullptr) {}
};

Node* removeZeroSum(Node* head) {
    Node* dummy = new Node(0);
    dummy->next = head;
    unordered_map<int, Node*> last;  // prefix sum -> last node with that sum
    int sum = 0;
    for (Node* cur = dummy; cur; cur = cur->next) {
        sum += cur->val;
        last[sum] = cur;  // overwrite keeps the LAST occurrence
    }
    sum = 0;
    for (Node* cur = dummy; cur; cur = cur->next) {
        sum += cur->val;
        cur->next = last[sum]->next;  // skip the zero-sum run
    }
    return dummy->next;
}

int main() {
    int vals[] = {3, 4, -7, 5, -6, 6};
    Node* head = nullptr;
    Node* tail = nullptr;
    for (int v : vals) {
        Node* n = new Node(v);
        if (!head) head = tail = n;
        else { tail->next = n; tail = n; }
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
