// Day 417: Remove all consecutive nodes summing to zero via prefix-sum + hashmap.
// Time O(n), Space O(n).
#include <bits/stdc++.h>
using namespace std;

struct ListNode {
    int val;
    ListNode* next;
    ListNode(int v): val(v), next(nullptr) {}
};

ListNode* removeZeroSum(ListNode* head) {
    ListNode dummy(0);
    dummy.next = head;
    unordered_map<int, ListNode*> seen; // prefix sum -> last node with that sum
    int prefix = 0;
    for (ListNode* node = &dummy; node; node = node->next) {
        prefix += node->val;
        seen[prefix] = node; // keep the latest node for this prefix sum
    }
    prefix = 0;
    for (ListNode* node = &dummy; node; node = node->next) {
        prefix += node->val;
        node->next = seen[prefix]->next; // skip the zero-sum run
    }
    return dummy.next;
}

int main() {
    int vals[] = {3, 4, -7, 5, -6, 6};
    ListNode dummy(0);
    ListNode* tail = &dummy;
    for (int v : vals) { tail->next = new ListNode(v); tail = tail->next; }
    ListNode* head = removeZeroSum(dummy.next);
    bool first = true;
    for (ListNode* n = head; n; n = n->next) {
        if (!first) cout << " -> ";
        cout << n->val;
        first = false;
    }
    cout << "\n";
    return 0;
}
