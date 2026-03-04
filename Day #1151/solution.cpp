// Day 1151: Palindrome linked list (singly).
// Find middle via slow/fast, reverse 2nd half, compare. O(n) time, O(1) space.
#include <bits/stdc++.h>
using namespace std;

struct Node { int val; Node* next; Node(int v): val(v), next(nullptr) {} };

bool isPalindrome(Node* head) {
    if (!head || !head->next) return true;
    Node *slow = head, *fast = head;
    while (fast->next && fast->next->next) { slow = slow->next; fast = fast->next->next; }
    Node *prev = nullptr, *cur = slow->next;
    while (cur) { Node* nx = cur->next; cur->next = prev; prev = cur; cur = nx; }
    Node *p1 = head, *p2 = prev;
    bool ok = true;
    while (p2) { if (p1->val != p2->val) { ok = false; break; } p1 = p1->next; p2 = p2->next; }
    return ok;
}

Node* build(vector<int> v) {
    Node dummy(0), *t = &dummy;
    for (int x : v) { t->next = new Node(x); t = t->next; }
    return dummy.next;
}

int main() {
    cout << boolalpha;
    cout << (isPalindrome(build({1, 4, 3, 4, 1})) ? "True" : "False") << "\n"; // True
    cout << (isPalindrome(build({1, 4})) ? "True" : "False") << "\n";          // False
    return 0;
}
