import fs from "fs";

fs.readFileSync("lines").
    toString().
    split("\n").
    forEach(line => console.log(line))