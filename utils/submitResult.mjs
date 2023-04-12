import assert from "node:assert/strict";
import { readFileSync, writeFileSync } from "fs";

const inputFileName = "final_pt_1.result.json";
const outputFileName = "output1.txt";

const inputFilePath = `outputs/${inputFileName}`;
const outputFilePath = `formatted_results/${outputFileName}`;

const jsonResult = JSON.parse(readFileSync(inputFilePath, "utf8"));

assert(Array.isArray(jsonResult));

let formattedResult = "";

for (let i = 0; i < jsonResult.length; i++) {
  const currentPosition = jsonResult[i];
  const nextPosition = jsonResult[i + 1];

  if (nextPosition === undefined) {
    // remove last space
    formattedResult = formattedResult.slice(0, formattedResult.length - 1);
    break;
  }

  const move = defineMoveBetweenPoints(currentPosition, nextPosition);

  formattedResult = formattedResult + move;
  formattedResult = formattedResult + " ";
}

let countMoves = formattedResult.split(" ").length;

// formattedResult = formattedResult + "\n";

writeFileSync(outputFilePath, formattedResult);

console.log(`Wrote solution with ${countMoves} moves.`);

function defineMoveBetweenPoints(x, y) {
  const [i1, j1] = x;
  const [i2, j2] = y;

  assert(typeof i1 === "number");
  assert(typeof i2 === "number");
  assert(typeof j1 === "number");
  assert(typeof j2 === "number");

  const iDifference = i2 - i1;
  const jDifference = j2 - j1;

  assert(Math.abs(iDifference) === 1 || Math.abs(jDifference) === 1);
  assert(!(Math.abs(iDifference) === 1 && Math.abs(jDifference) === 1));
  assert(Math.abs(iDifference) === 0 || Math.abs(jDifference) === 0);

  switch (iDifference) {
    case 1:
      return "D";
    case -1:
      return "U";
    case 0:
      break;
    default:
      throw Error("Invalid iDifference");
  }

  switch (jDifference) {
    case 1:
      return "R";
    case -1:
      return "L";
    default:
      throw Error("Invalid jDifference");
  }
}
