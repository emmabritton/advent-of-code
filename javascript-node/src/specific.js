const resources = require("./resources");

module.exports = function(run, year, day) {
    console.log(`Year ${year} Day ${day}`);

    const results = run(year, day);

    if (results == null) {
        console.log("Not implemented");
    } else {
        checkAnswer(results[0], year, day, 1);
        checkAnswer(results[1], year, day, 2);
    }
}

function checkAnswer(result, year, day, part) {
    try {
        const answer = resources.readAnswer(year, day, part);
        if (answer == null) {
            console.log(`No answer provided for part ${part}, calculated was '${result}'`);
        } else {
            if (result === answer) {
                console.log(`Provided answer and result match for part ${part}, both are ${result}`);
            } else {
                console.log(`Provided answer and result mismatch for part ${part}, answer: '${answer}' result: '${result}'`);
            }
        }
    } catch (e) {
        console.log(`Unable to open answer for part ${part}: ${e}`);
    }

}