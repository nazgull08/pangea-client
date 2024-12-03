const fs = require("fs");

// clean package.json
const packageJson = JSON.parse(fs.readFileSync("package.json", "utf8"));

// remove example dependencies
delete packageJson.devDependencies["pangea-client"];
delete packageJson.devDependencies["apache-arrow"];
delete packageJson.devDependencies["ts-jest"];
delete packageJson.devDependencies["ts-node"];

// remove example scripts
for (const key in packageJson.scripts) {
  if (key.startsWith("examples:")) {
    delete packageJson.scripts[key];
  }
}

// delete support scripts
delete packageJson.scripts["build-dev"];
delete packageJson.scripts["build-dev:swc"];
delete packageJson.scripts["prepare"];
delete packageJson.scripts["prepack"];
delete packageJson.scripts["pack"];
delete packageJson.scripts["postpack"];

fs.writeFileSync("package.json", JSON.stringify(packageJson, null, 2));

const readmePath = "README.md";

const lines = fs.readFileSync(readmePath, "utf-8").split("\n");

// write back only the first 28 lines
fs.writeFileSync(readmePath, lines.slice(0, 28).join("\n"));
