const operation = function(operationValue) {
  return Function('number', `'use strict'; return "${operationValue}" + number`)
}

const number = function(numberValue) {
  return Function('...operation', `'use strict'; let result; if(operation.length) { result = parse(value + operation); } else { result = value; } return result; `)
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

const plus = operation("+");
const minus = operation("-");
const times = operation("*");
const dividedBy = operation("/");
