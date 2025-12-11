import { computeAverage } from "./index";

const data = [1.0, 2.0, 3.0, 4.0, 5.0];
const result = computeAverage(data);
console.log(`Average of [${data}]: ${result}`);

const emptyResult = computeAverage([]);
console.log(`Average of []: ${emptyResult}`);

