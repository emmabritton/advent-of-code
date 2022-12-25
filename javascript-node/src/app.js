const runAll = require("./runner")
const runSpecific = require("./specific")
const year2015 = require('./y2015/year2015')
const year2016 = require('./y2016/year2016')

if (process.argv.length === 4) {
    const year = parseInt(process.argv[2]);
    const day = parseInt(process.argv[3]);
    if (!isNaN(year) && !isNaN(day)) {
        runSpecific(run, year, day);
    } else {
        runAll(run);
    }
} else {
    runAll(run);
}

function run(year, day) {
    switch (year) {
        case 2015:
            return year2015(day);
        case 2016:
            return year2016(day);
        case 2017:
        case 2018:
        case 2019:
        case 2020:
        case 2021:
        case 2022:
            return null;
        default:
            throw new Error(`Invalid year ${year}`)
    }
}