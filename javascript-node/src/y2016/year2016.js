const resources = require("../resources");
const day1 = require("./day1");

module.exports = function(day) {
    switch (day) {
        case 1: return day1(resources.readInput(2016, 1));
        case 2:
        case 3:
        case 4:
        case 5:
        case 6:
        case 7:
        case 8:
        case 9:
        case 10:
        case 11:
        case 12:
        case 13:
        case 14:
        case 15:
        case 16:
        case 17:
        case 18:
        case 19:
        case 20:
        case 21:
        case 22:
        case 23:
        case 24:
        case 25:
            return null;
        default:
            throw new Error(`Invalid day for 2016: ${day}`);
    }
}