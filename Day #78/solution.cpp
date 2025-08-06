// Day 78: Merge k sorted linked lists using a min-heap.
// Time O(N log k) where N total nodes, Space O(k).
#include <bits/stdc++.h>
using namespace std;

struct ListNode {
    int val; ListNode* next;
    ListNode(int v) : val(v), next(nullptr) {}
};

ListNode* mergeKLists(vector<ListNode*>& lists) {
    auto cmp = [](ListNode* a, ListNode* b) { return a->val > b->val; };
    priority_queue<ListNode*, vector<ListNode*>, decltype(cmp)> pq(cmp);
    for (auto l : lists) if (l) pq.push(l);
    ListNode dummy(0), *tail = &dummy;
    while (!pq.empty()) {
        ListNode* n = pq.top(); pq.pop();
        tail->next = n; tail = n;
        if (n->next) pq.push(n->next);
    }
    return dummy.next;
}

ListNode* build(vector<int> v) {
    ListNode dummy(0), *t = &dummy;
    for (int x : v) { t->next = new ListNode(x); t = t->next; }
    return dummy.next;
}

int main() {
    vector<ListNode*> lists = {build({1,4,5}), build({1,3,4}), build({2,6})};
    ListNode* m = mergeKLists(lists);
    for (; m; m = m->next) cout << m->val << (m->next ? " -> " : "\n");
    // 1 -> 1 -> 2 -> 3 -> 4 -> 4 -> 5 -> 6
    return 0;
}
