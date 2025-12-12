// Quack (push/pop left, pull right) via three stacks.
// On underflow of one side, rebalance by moving half from the other side using a temp stack.
// Amortized O(1) per operation, O(1) extra memory beyond the three stacks.
import java.util.*;

public class Solution {
    static class Quack {
        Deque<Integer> front = new ArrayDeque<>(); // peek = leftmost
        Deque<Integer> back  = new ArrayDeque<>(); // peek = rightmost
        Deque<Integer> temp  = new ArrayDeque<>();

        void push(int x){ front.push(x); }

        void refillFront(){
            int n = back.size();
            int leftCount = (n + 1) / 2, rightCount = n - leftCount;
            for(int i=0;i<rightCount;i++) temp.push(back.pop());
            for(int i=0;i<leftCount;i++) front.push(back.pop());
            while(!temp.isEmpty()) back.push(temp.pop());
        }
        void refillBack(){
            int n = front.size();
            int rightCount = (n + 1) / 2, leftCount = n - rightCount;
            for(int i=0;i<leftCount;i++) temp.push(front.pop());
            for(int i=0;i<rightCount;i++) back.push(front.pop());
            while(!temp.isEmpty()) front.push(temp.pop());
        }

        int pop(){
            if(front.isEmpty()) refillFront();
            if(front.isEmpty()) throw new RuntimeException("empty");
            return front.pop();
        }
        int pull(){
            if(back.isEmpty()) refillBack();
            if(back.isEmpty()) throw new RuntimeException("empty");
            return back.pop();
        }
    }

    public static void main(String[] args){
        Quack q = new Quack();
        q.push(1); q.push(2); q.push(3);
        System.out.println("pop: "  + q.pop());  // 3
        System.out.println("pull: " + q.pull()); // 1
        q.push(4);
        System.out.println("pull: " + q.pull()); // 2
        System.out.println("pop: "  + q.pop());  // 4
    }
}
