function productFib(prod) {
  let fibOne = 0;
  let fibTwo = 1;
  let fibThree = 0;
  while(1) {
    fibThree = fibOne + fibTwo;

    if(fibOne*fibTwo === prod) {
      return [fibOne, fibTwo, true];
    } else if(fibOne*fibTwo < prod && fibTwo*fibThree > prod) {
      return [fibTwo, fibThree, false];
    }

    fibOne = fibTwo;
    fibTwo = fibThree;
  }
}

