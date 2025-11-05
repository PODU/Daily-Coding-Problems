// Merge k sorted linked lists using a min-heap of list heads.
// Time: O(N log k), Space: O(k) for the heap.
#include <bits/stdc++.h>
using namespace std;

struct ListNode {
    int val;
    ListNode* next;
    ListNode(int v) : val(v), next(nullptr) {}
};

ListNode* buildList(const vector<int>& v) {
    ListNode dummy(0);
    ListNode* cur = &dummy;
    for (int x : v) { cur->next = new ListNode(x); cur = cur->next; }
    return dummy.next;
}

ListNode* mergeKLists(vector<ListNode*>& lists) {
    auto cmp = [](ListNode* a, ListNode* b) { return a->val > b->val; };
    priority_queue<ListNode*, vector<ListNode*>, decltype(cmp)> pq(cmp);
    for (ListNode* n : lists) if (n) pq.push(n);
    ListNode dummy(0);
    ListNode* tail = &dummy;
    while (!pq.empty()) {
        ListNode* node = pq.top(); pq.pop();
        tail->next = node; tail = node;
        if (node->next) pq.push(node->next);
    }
    return dummy.next;
}

int main() {
    vector<ListNode*> lists = {
        buildList({1, 4, 5}),
        buildList({1, 3, 4}),
        buildList({2, 6})
    };
    ListNode* merged = mergeKLists(lists);
    bool first = true;
    for (ListNode* n = merged; n; n = n->next) {
        if (!first) cout << ' ';
        cout << n->val;
        first = false;
    }
    cout << '\n';
    return 0;
}
