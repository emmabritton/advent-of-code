const path = require('path');
const fs = require('fs');

const resDir = path.normalize(process.cwd() + "/../resources");
const answersPath = path.join(resDir, "answers");
const inputsPath = path.join(resDir, "inputs");

module.exports.readAnswer = function(year, day, part) {
    const file = path.join(answersPath, year.toString(), `day${day}p${part}`);
    if (fs.existsSync(file)) {
        return fs.readFileSync(file, {encoding: "utf8"}).trim();
    } else {
        return null;
    }
}

module.exports.readInput = function(year, day) {
    const file = path.join(inputsPath, year.toString(), `day${day}`);
    if (fs.existsSync(file)) {
        return fs.readFileSync(file, {encoding: "utf8"});
    } else {
        throw new Error(`No input file found for ${year}/${day}`);
    }
}