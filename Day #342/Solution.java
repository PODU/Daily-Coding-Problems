// Custom left fold (reduce). acc=init; for each x: acc=f(acc,x). O(n) time, O(1) space.
import java.util.*;
import java.util.function.BiFunction;

public class Solution {
    static <T, A> A myReduce(List<T> arr, BiFunction<A, T, A> f, A init){
        A acc = init;
        for(T x : arr) acc = f.apply(acc, x);
        return acc;
    }

    static int add(int a, int b){ return a + b; }

    static int sum(List<Integer> lst){
        return myReduce(lst, (a, x) -> add(a, x), 0);
    }

    public static void main(String[] args){
        List<Integer> lst = Arrays.asList(1,2,3,4,5);
        System.out.println(sum(lst));
    }
}
