// Evaluate +/- expression with parentheses using a sign stack (Basic Calculator).
// Single linear scan; parentheses handled by pushing the running result and sign.
// Time: O(n), Space: O(n).
#include <bits/stdc++.h>
using namespace std;

int evaluate(const string& s){
    long long result = 0;
    int sign = 1;
    stack<int> st; // pushes (prevResult, prevSign)
    int i = 0, n = s.size();
    while(i < n){
        char c = s[i];
        if(isdigit(c)){
            long long num = 0;
            while(i < n && isdigit(s[i])){ num = num*10 + (s[i]-'0'); i++; }
            result += sign * num;
            continue;
        } else if(c == '+'){ sign = 1; }
        else if(c == '-'){ sign = -1; }
        else if(c == '('){ st.push(result); st.push(sign); result = 0; sign = 1; }
        else if(c == ')'){ int s2 = st.top(); st.pop(); int r2 = st.top(); st.pop(); result = r2 + s2*result; }
        i++;
    }
    return (int)result;
}

int main(){
    cout << evaluate("-1 + (2 + 3)") << endl; // 4
    return 0;
}
