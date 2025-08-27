// Interleave first half with reversed second half using ONE auxiliary queue.
// Result (bottom->top) = arr[0],arr[n-1],arr[1],arr[n-2],...  O(n^2) time, O(n) space.
import java.util.*;

public class Solution {
    static List<Integer> interleave(int[] arr){
        Deque<Integer> stack = new ArrayDeque<>(); // push/pop = top operations
        for(int x : arr) stack.push(x);            // top = last element
        Queue<Integer> q = new LinkedList<>();     // offer/poll = enqueue/dequeue

        while(!stack.isEmpty()) q.offer(stack.pop()); // A: stack -> queue
        while(!q.isEmpty()) stack.push(q.poll());     // B: queue -> stack (reverse)
        while(!stack.isEmpty()) q.offer(stack.pop()); // C: stack -> queue (front..back = bottom..top)

        boolean takeFront = true;
        while(!q.isEmpty()){
            if(takeFront){ stack.push(q.poll()); }
            else{
                for(int k=q.size()-1; k>0; k--) q.offer(q.poll()); // rotate back to front
                stack.push(q.poll());
            }
            takeFront = !takeFront;
        }
        List<Integer> res = new ArrayList<>();
        while(!stack.isEmpty()) res.add(stack.pop()); // top -> bottom
        Collections.reverse(res);                     // bottom -> top
        return res;
    }

    public static void main(String[] args){
        System.out.println(interleave(new int[]{1,2,3,4,5}));
        System.out.println(interleave(new int[]{1,2,3,4}));
    }
}
