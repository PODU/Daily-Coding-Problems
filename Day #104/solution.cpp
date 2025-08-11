// Day 104: Linked-list palindrome (singly or doubly). Find middle, reverse second
// half, compare both halves. O(n) time, O(1) extra space.
#include <bits/stdc++.h>
using namespace std;

struct Node { int val; Node* next; Node(int v): val(v), next(nullptr) {} };

bool isPalindrome(Node* head) {
    Node *slow = head, *fast = head;
    while (fast && fast->next) { slow = slow->next; fast = fast->next->next; }
    Node* prev = nullptr;
    while (slow) { Node* nx = slow->next; slow->next = prev; prev = slow; slow = nx; }
    Node *l = head, *r = prev;
    while (r) { if (l->val != r->val) return false; l = l->next; r = r->next; }
    return true;
}

Node* build(vector<int> vals) {
    Node dummy(0); Node* cur = &dummy;
    for (int v : vals) { cur->next = new Node(v); cur = cur->next; }
    return dummy.next;
}

int main() {
    cout << boolalpha;
    cout << isPalindrome(build({1, 4, 3, 4, 1})) << "\n"; // true
    cout << isPalindrome(build({1, 4})) << "\n";          // false
    return 0;
}
