// Day 115: Subtree check via recursive structural match. O(|s|*|t|) worst case.
#include <bits/stdc++.h>
using namespace std;
struct Node { int val; Node *l, *r; Node(int v):val(v),l(nullptr),r(nullptr){} };

bool same(Node* a, Node* b){
    if(!a && !b) return true;
    if(!a || !b || a->val != b->val) return false;
    return same(a->l, b->l) && same(a->r, b->r);
}
bool isSubtree(Node* s, Node* t){
    if(!t) return true;
    if(!s) return false;
    if(same(s, t)) return true;
    return isSubtree(s->l, t) || isSubtree(s->r, t);
}
int main(){
    Node* s = new Node(3);
    s->l = new Node(4); s->r = new Node(5);
    s->l->l = new Node(1); s->l->r = new Node(2);

    Node* t = new Node(4); t->l = new Node(1); t->r = new Node(2);
    Node* u = new Node(4); u->l = new Node(0);

    cout << (isSubtree(s, t) ? "true" : "false") << "\n"; // true
    cout << (isSubtree(s, u) ? "true" : "false") << "\n"; // false
    return 0;
}
