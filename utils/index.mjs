import fs from "fs";
import readline from "readline";

const inputFile = "utils/input.txt";

const outputFile = "utils/output.json";

const readStream = fs.createReadStream(inputFile);
const rl = readline.createInterface({
  input: readStream,
  crlfDelay: Infinity,
});

const writeStream = fs.createWriteStream(outputFile);

const formattedLines = [];

rl.on("line", (line) => {
  const replaced = line.replace(/ /g, ",");

  const formatted = `[${replaced}],`;

  formattedLines.push(formatted);
});

rl.on("close", () => {
  let outputJson = `[${formattedLines.join("\n").slice(0, -1)}]`;

  if (outputJson.includes("3")) {
    outputJson = outputJson.replace("3", "2");
    console.log("Replaced 3 by 2");
  }

  if (outputJson.includes("4")) {
    console.log("Replaced 4 by 3");
    outputJson = outputJson.replace("4", "3");
  }

  writeStream.write(outputJson);
  writeStream.end();
  console.log("File processed successfully.");
});
