// Day 869: Is a linked list a palindrome (works for singly linked; doubly trivial).
// Approach: find middle (slow/fast), reverse second half, compare both halves. O(1) space.
// Time: O(n), Space: O(1).
#include <bits/stdc++.h>
using namespace std;

struct Node { int val; Node* next; Node(int v):val(v),next(nullptr){} };

Node* build(vector<int> vals) {
    Node* head = nullptr; Node* tail = nullptr;
    for (int v : vals) {
        Node* n = new Node(v);
        if (!head) head = tail = n;
        else { tail->next = n; tail = n; }
    }
    return head;
}

bool isPalindrome(Node* head) {
    Node *slow = head, *fast = head;
    while (fast && fast->next) { slow = slow->next; fast = fast->next->next; }
    // reverse second half
    Node* prev = nullptr;
    while (slow) { Node* nx = slow->next; slow->next = prev; prev = slow; slow = nx; }
    Node *a = head, *b = prev;
    while (b) { if (a->val != b->val) return false; a = a->next; b = b->next; }
    return true;
}

int main() {
    cout << (isPalindrome(build({1,4,3,4,1})) ? "True" : "False") << "\n"; // True
    cout << (isPalindrome(build({1,4})) ? "True" : "False") << "\n";       // False
    return 0;
}
