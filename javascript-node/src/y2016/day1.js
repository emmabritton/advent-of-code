module.exports = function (input) {
    const steps = input.split(",")
        .map(mov => parse(mov.trim()));

    let visited = [];
    let pos = [0, 0];
    let repeat = null;
    let dir = NORTH;

    for (const step of steps) {
        dir = next(dir, step[0]);
        for (let i = 1; i <= step[1]; i++) {
            const newCoord = walk(pos, dir, i);
            if (repeat == null && contains(visited, newCoord)) {
                repeat = newCoord;
            } else {
                visited.push(newCoord);
            }
        }
        pos = walk(pos, dir, step[1]);
    }

    return [distance(pos).toString(), distance(repeat || [0, 0]).toString()];
}

function contains(list, newCoord) {
    for (const coord of list) {
        if (coord[0] === newCoord[0] && coord[1] === newCoord[1]) {
            return true
        }
    }
    return false
}

function walk(pos, dir, dist) {
    switch (dir) {
        case NORTH:
            return [pos[0], pos[1] + dist]
        case SOUTH:
            return [pos[0], pos[1] - dist]
        case EAST:
            return [pos[0] + dist, pos[1]]
        case WEST:
            return [pos[0] - dist, pos[1]]
    }
}

function distance(coord) {
    return Math.abs(coord[0]) + Math.abs(coord[1])
}

function parse(input) {
    if (input.length === 1) throw new Error(`Too short: ${input}`);
    const number = parseInt(input.substr(1))
    if (isNaN(number)) {
        throw new Error(`Invalid number: ${input}`);
    }
    if (input[0] === RIGHT || input[0] === LEFT) {
        return [input[0], number]
    }
    throw new Error(`Invalid letter: ${input}`)
}

function next(current, turn) {
    switch (current) {
        case NORTH:
            return turn === LEFT ? WEST : EAST
        case SOUTH:
            return turn === LEFT ? EAST : WEST
        case EAST:
            return turn === LEFT ? NORTH : SOUTH
        case WEST:
            return turn === LEFT ? SOUTH : NORTH
    }
}

const LEFT = 'L';
const RIGHT = 'R';
const NORTH = 'N';
const SOUTH = 'S';
const WEST = 'W';
const EAST = 'E';