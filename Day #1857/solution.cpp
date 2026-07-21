// Day 1857: Evaluate Reverse Polish Notation.
// Stack: push numbers, pop two on operator and apply. O(n) time, O(n) space.
#include <bits/stdc++.h>
using namespace std;

long long evalRPN(const vector<string>& tokens) {
    vector<long long> st;
    auto isOp = [](const string& t) { return t == "+" || t == "-" || t == "*" || t == "/"; };
    for (const string& t : tokens) {
        if (isOp(t)) {
            long long b = st.back(); st.pop_back();
            long long a = st.back(); st.pop_back();
            long long r = 0;
            if (t == "+") r = a + b;
            else if (t == "-") r = a - b;
            else if (t == "*") r = a * b;
            else r = a / b; // C++ integer division truncates toward zero
            st.push_back(r);
        } else {
            st.push_back(stoll(t));
        }
    }
    return st.back();
}

int main() {
    vector<string> tokens = {"15","7","1","1","+","-","/","3","*","2","1","1","+","+","-"};
    cout << evalRPN(tokens) << "\n"; // 5
    return 0;
}
