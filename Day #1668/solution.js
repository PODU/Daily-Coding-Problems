// Iterate counting integers; pick those whose digit sum == 10. O(answer) time, O(1) space.
function digitSum(x){let s=0;while(x>0){s+=x%10;x=Math.floor(x/10);}return s;}
function nthPerfect(n){let x=0,c=0;while(c<n){++x;if(digitSum(x)===10)++c;}return x;}
console.log(nthPerfect(1));
console.log(nthPerfect(2));
