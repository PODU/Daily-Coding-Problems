// Evaluate Reverse Polish Notation with a stack. Time O(n), Space O(n).
#include <bits/stdc++.h>
using namespace std;

long long evalRPN(const vector<string>& tokens){
    vector<long long> st;
    for(const string& t : tokens){
        if(t == "+" || t == "-" || t == "*" || t == "/"){
            long long b = st.back(); st.pop_back();
            long long a = st.back(); st.pop_back();
            long long r = 0;
            if(t == "+") r = a + b;
            else if(t == "-") r = a - b;
            else if(t == "*") r = a * b;
            else r = a / b; // truncates toward zero
            st.push_back(r);
        } else {
            st.push_back(stoll(t));
        }
    }
    return st.back();
}

int main(){
    vector<string> tokens = {"15","7","1","1","+","-","/","3","*","2","1","1","+","+","-"};
    cout << evalRPN(tokens) << "\n";
    return 0;
}
