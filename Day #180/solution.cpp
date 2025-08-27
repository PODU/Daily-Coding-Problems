// Interleave first half with reversed second half using ONE auxiliary queue.
// Result (bottom->top) = arr[0],arr[n-1],arr[1],arr[n-2],...  O(n^2) time, O(n) space.
#include <bits/stdc++.h>
using namespace std;

// stack = vector with top at back (push_back/pop_back); queue = std::queue (push/pop/front).
vector<int> interleave(vector<int> stack){
    queue<int> q;
    while(!stack.empty()){ q.push(stack.back()); stack.pop_back(); } // A: stack -> queue
    while(!q.empty()){ stack.push_back(q.front()); q.pop(); }        // B: queue -> stack (reverse)
    while(!stack.empty()){ q.push(stack.back()); stack.pop_back(); } // C: stack -> queue (now front..back = bottom..top)
    bool takeFront = true;
    while(!q.empty()){
        if(takeFront){ stack.push_back(q.front()); q.pop(); }
        else{
            for(int k=(int)q.size()-1; k>0; k--){ q.push(q.front()); q.pop(); } // rotate back element to front
            stack.push_back(q.front()); q.pop();
        }
        takeFront = !takeFront;
    }
    return stack;
}

void pr(const vector<int>& v){
    cout << "[";
    for(size_t i=0;i<v.size();i++){ cout << v[i]; if(i+1<v.size()) cout << ", "; }
    cout << "]\n";
}

int main(){
    pr(interleave({1,2,3,4,5}));
    pr(interleave({1,2,3,4}));
    return 0;
}
