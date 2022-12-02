function zeros(n) {
  let sum = 0;
  for(let i=1; i <= Math.log(n)/Math.log(5); i++) {
    sum += Math.floor(n/Math.pow(5,i));
  }
  return sum;
}

