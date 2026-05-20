// Remove all consecutive nodes summing to zero using prefix sums + hash map.
// A prefix sum seen before means the span between is zero-sum and is excised.
// Time O(n), Space O(n).
#include <bits/stdc++.h>
using namespace std;

struct Node {
    int val;
    Node* next;
    Node(int v) : val(v), next(nullptr) {}
};

Node* removeZeroSum(Node* head) {
    Node dummy(0);
    dummy.next = head;
    unordered_map<int, Node*> seen; // prefix sum -> node
    int sum = 0;
    for (Node* p = &dummy; p; p = p->next) {
        sum += p->val;
        seen[sum] = p; // last node achieving this prefix sum
    }
    sum = 0;
    for (Node* p = &dummy; p; p = p->next) {
        sum += p->val;
        p->next = seen[sum]->next;
    }
    return dummy.next;
}

int main() {
    int vals[] = {3, 4, -7, 5, -6, 6};
    Node* head = nullptr; Node** tail = &head;
    for (int v : vals) { *tail = new Node(v); tail = &(*tail)->next; }
    head = removeZeroSum(head);
    bool first = true;
    for (Node* p = head; p; p = p->next) {
        if (!first) cout << " ";
        cout << p->val;
        first = false;
    }
    cout << "\n";
    return 0;
}
