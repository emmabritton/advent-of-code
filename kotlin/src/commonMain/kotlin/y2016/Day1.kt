package y2016

import kotlin.math.abs

fun runY2016D1(input: String): Pair<String, String> {
    val steps = input.split(",")
        .map { parse(it.trim()) }

    val (distance, firstRepeat) = travel(steps)

    return Pair(distance.toString(), firstRepeat.toString())
}

fun parse(input: String): Move {
    if (input.length == 1) {
        throw IllegalArgumentException("Too short: $input")
    }
    val number = input.substring(1).toIntOrNull() ?: throw IllegalArgumentException("Invalid number: $input")
    return when (input.first()) {
        'R' -> Move.Right(number)
        'L' -> Move.Left(number)
        else -> throw IllegalArgumentException("Invalid letter: $input")
    }
}

fun travel(moves: List<Move>): Pair<Int, Int> {
    val visited = HashSet<Coord>()
    val pos = Coord(0,0)
    var dir = Direction.North
    var firstRepeat: Coord? = null
    for (move in moves) {
        dir = dir.update(move)
        val walked = pos.walk(dir, move)
        for (step in walked) {
            if (firstRepeat == null) {
                if (visited.contains(step)) {
                    firstRepeat = step
                }
                println("Visited ${step.x},${step.y}");
                visited.add(step)
            }
        }
    }
    return Pair(pos.distance(), firstRepeat?.distance() ?: 0)
}

data class Coord(
    var x: Int,
    var y: Int
) {
    fun distance(): Int {
        return abs(x) + abs(y)
    }

    fun walk(dir: Direction, move: Move): List<Coord> {
        var list = listOf<Coord>()
        when (dir) {
            Direction.North -> {
                list = (1..move.dist).map { Coord(this.x, this.y + it) }
                this.y += move.dist
            }
            Direction.East -> {
                list = (1..move.dist).map { Coord(this.x + it, this.y) }
                this.x += move.dist
            }
            Direction.South -> {
                list = (1..move.dist).map { Coord(this.x, this.y - it) }
                this.y -= move.dist
            }
            Direction.West -> {
                list = (1..move.dist).map { Coord(this.x - it, this.y) }
                this.x -= move.dist
            }
        }
        return list
    }
}

enum class Direction {
    North,
    East,
    South,
    West;

    fun update(move: Move): Direction {
        return when (this) {
            North -> if (move is Move.Left) West else East
            East -> if (move is Move.Left) North else South
            South -> if (move is Move.Left) East else West
            West -> if (move is Move.Left) South else North
        }
    }
}

sealed class Move(val dist: Int) {
    class Left(dist: Int): Move(dist)
    class Right(dist: Int): Move(dist)
}

