const operation = function(number) {
  const operator = "+";
  return operator + number
}
const number = function(...operation) {
  const value = "0";
  let result = null;
    if(operation) {
      result = Function(`'use strict'; return ($value + ${operation})`)();
    }
    else {
      result = value;
    }
  return result; 
}

function parse(str) {
  return Function(`'use strict'; return (${str})`)()
}

const zero = function (...operation) {
  const value = "0";
  let result = null;
    if(operation.length) {
      result = parse(value + operation);
    }
    else {
      result = value;
    }
  return result; 
}

const one = function (...operation) {
  const value = "1";
  let result = null;
    if(operation.length) {
      // result = value + operation;
      // result = Function("'use strict'; return value + operation;")();
      result = parse(value + operation);
    }
    else {
      result = value;
    }
  return result; 
}

const two = function (...operation) {
  const value = "2";
  let result = null;
    if(operation.length) {
      result = parse(value + operation);
    }
    else {
      result = value;
    }
  return result; 
}

// function three() {}
// function four() {}
// function five() {}
// function six() {}
// function seven() {}
// function eight() {}
// function nine() {}

const plus = function(number) {
  const operator = "+";
  return `${operator}${number}`;
}
// function minus() {}
// function times() {}
// function dividedBy() {}
