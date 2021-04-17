module.exports = function(input) {
    const result = input.split("\n")
        .map(value => parse(value))
        .map(result => calc(result))
        .reduce((acc, value) => [acc[0] + value[0], acc[1] + value[1]]);

    return [result[0].toString(), result[1].toString()];
}

function parse(value) {
    const numbers = value.split("x").map(value => parseInt(value));
    const count = numbers.filter(value => !isNaN(value)).length;
    if (count === 3) {
        return [numbers[0], numbers[1], numbers[2]];
    } else {
        throw new Error(`Invalid number of sides: $value`);
    }
}

function calc(input) {
    const idx = input.indexOf(Math.max(...input));
    let smallerSides = [...input];
    smallerSides.splice(idx, 1);

    const paper = (2 * input[0] * input[1]) + (2 * input[1] * input[2]) + (2 * input[0] * input[2]) + (smallerSides[0] * smallerSides[1]);
    const ribbon = (smallerSides[0] + smallerSides[0] + smallerSides[1] + smallerSides[1]) + (input[0] * input[1] * input[2]);

    return [paper, ribbon];
}