const { execSync } = require("child_process");

execSync("git restore README.md", { stdio: "inherit" });
execSync("git restore package.json", { stdio: "inherit" });
