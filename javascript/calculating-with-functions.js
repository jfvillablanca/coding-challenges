const operation = function(operationValue) {
  return Function('number', `'use strict'; return "${operationValue}" + number`)
}

const number = function(numberValue) {
  return Function('...operation', `
      let result; 
      if(arguments.length !== 0) { 
        // console.log(typeof (${numberValue} + operation));
        result = Math.floor(eval(${numberValue} + operation));
      } else { 
        result = "${numberValue}"; 
      } 
      return result;`);
}

const zero = number("0");
const one = number("1");
const two = number("2");
const three = number("3");
const four = number("4");
const five = number("5");
const six = number("6");
const seven = number("7");
const eight = number("8");
const nine = number("9");

const plus = operation("+");
const minus = operation("-");
const times = operation("*");
const dividedBy = operation("/");
