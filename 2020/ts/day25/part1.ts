import { readFile } from "../utils.ts";

const SUBJECT_NUMBER = 7;
const MOD = 20201227;

const [cardPublicKey, doorPublicKey] = (await readFile(import.meta.url))
  .split("\n").map(Number);

const transform = (value: number, subjectNumber = SUBJECT_NUMBER): number =>
  (value * subjectNumber) % MOD;

let loopValue = 1;
let encryptionKey = 1;
while (loopValue !== cardPublicKey) {
  loopValue = transform(loopValue);
  encryptionKey = transform(encryptionKey, doorPublicKey);
}

console.log(encryptionKey);
