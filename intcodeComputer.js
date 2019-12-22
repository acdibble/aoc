const processIntcodes = (ops, inputIntcodes) => {
  const intcodes = inputIntcodes.slice();
  let relativeBase = 0;
  let i = 0;
  while (i < intcodes.length) {
    let inputCode = intcodes[i];
    if (inputCode === 99) {
      break;
    }

    const padded = [0, 0, 0, 0, 0];
    for (let j = 4; j >= 0; j--) {
      padded[j] = inputCode % 10;
      inputCode = Math.floor(inputCode / 10);
    }
    const [relativeMode, param2Mode, param1Mode, , opcode] = padded;
    let param1 = intcodes[++i];
    let operand1 = null;
    if (param1Mode === 1) {
      operand1 = param1;
    } else if (param1Mode === 2) {
      if (opcode === 3) {
        param1 = relativeBase + param1;
      }
      operand1 = intcodes[relativeBase + param1];
    } else {
      operand1 = intcodes[param1];
    }
    let operand2 = null;
    if (opcode !== 3 && opcode !== 4 && opcode !== 9) {
      const param2 = intcodes[++i];
      if (param2Mode === 1) {
        operand2 = param2;
      } else if (param2Mode === 2) {
        operand2 = intcodes[relativeBase + param2];
      } else {
        operand2 = intcodes[param2];
      }
    } else if (opcode === 9) {
      operand2 = relativeBase;
    }

    const result = ops[opcode](operand1, operand2);
    if (opcode === 1 || opcode === 2 || opcode === 7 || opcode === 8) {
      const index = intcodes[++i] + (relativeMode === 2 ? relativeBase : 0);
      intcodes[index] = result;
    } else if (opcode === 3) {
      intcodes[param1] = result;
    } else if (opcode === 9) {
      relativeBase = result;
    }

    if ((opcode === 5 || opcode === 6) && result !== -1) {
      i = result;
    } else {
      i += 1;
    }
  }
};

const intcodeComputer = (inputHandler, outputHandler) => {
  const ops = {
    // ADD
    1(a = 0, b = 0) {
      return a + b;
    },
    // MULT
    2(a = 0, b = 0) {
      return a * b;
    },
    // INPUT
    3() {
    },
    // OUTPUT
    4(a) {
    },
    // NE
    5(a = 0, b = 0) {
      return a !== 0 ? b : -1;
    },
    // EQ
    6(a = 0, b = 0) {
      return a === 0 ? b : -1;
    },
    // LT
    7(a = 0, b = 0) {
      return a < b ? 1 : 0;
    },
    // EQ
    8(a = 0, b = 0) {
      return a === b ? 1 : 0;
    },
    9(a, relativeBase) {
      return a + relativeBase;
    },
  };

  if (inputHandler) ops[3] = inputHandler;
  if (outputHandler) ops[4] = outputHandler;

  return (intcodes) => processIntcodes(ops, intcodes);
};

module.exports = intcodeComputer;
