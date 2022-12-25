const resources = require("./resources");

module.exports = function(run) {
    for (let year = 2015; year <= 2022; year++) {
        console.log(`\n\n${year}\n`);
        for (let day = 1; day <= 25; day++) {
            let result = runDay(run, year, day);
            let padded = day.toString().padStart(2, " ");
            console.log(`${padded}   ${result}`);
        }
    }
}

function runDay(run, year, day) {
    try {
        const results = run(year, day);
        if (results == null) {
            return "Not implemented";
        } else {
            const answer1 = getAnswer(results[0], year, day, 1);
            const answer2 = getAnswer(results[1], year, day, 2);
            return `[${answer1}] [${answer2}]`;
        }
    } catch (e) {
        return "Error running";
    }
}

function getAnswer(result, year, day, part) {
    try {
        const answer = resources.readAnswer(year, day, part);
        if (answer == null) {
            return "?";
        } else {
            if (answer === result) {
                return "✓";
            } else {
                return "✖";
            }
        }
    } catch {
        return "⚠";
    }
}