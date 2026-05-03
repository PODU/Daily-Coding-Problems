// Evaluate Reverse Polish Notation using a stack.
// Time: O(n), Space: O(n).
#include <bits/stdc++.h>
using namespace std;

int main() {
    // Tokens: ints and operator strings.
    vector<string> tokens = {"15","7","1","1","+","-","/","3","*","2","1","1","+","+","-"};
    stack<long long> st;
    for (const string& t : tokens) {
        if (t=="+"||t=="-"||t=="*"||t=="/") {
            long long b = st.top(); st.pop();
            long long a = st.top(); st.pop();
            long long r = 0;
            if (t=="+") r = a+b;
            else if (t=="-") r = a-b;
            else if (t=="*") r = a*b;
            else r = a/b; // integer division truncates toward zero
            st.push(r);
        } else {
            st.push(stoll(t));
        }
    }
    cout << st.top() << endl;
    return 0;
}
