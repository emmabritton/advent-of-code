package y2015

fun runY2015D2(input: String): Pair<String, String> {
    val result = input.lines()
        .map { parse(it) }
        .map { calc(it) }
        .reduce { acc, pair -> Pair(acc.first + pair.first, acc.second + pair.second) }
    return Pair(result.first.toString(), result.second.toString())
}

private fun parse(input: String): List<Int> {
    val list = input.split("x").mapNotNull { it.toIntOrNull() }
    if (list.size == 3) {
        return list
    } else {
        throw IllegalStateException("Invalid number of sides: $input")
    }
}

private fun calc(input: List<Int>): Pair<Int, Int> {
    val highest = input.maxOrNull()!!
    val smallerSides = ArrayList(input).apply { remove(highest) }

    val paper = (2 * input[0] * input[1]) + (2 * input[1] * input[2]) + (2 * input[0] * input[2]) + (smallerSides[0] * smallerSides[1])
    val ribbon = (smallerSides[0] + smallerSides[0] + smallerSides[1] + smallerSides[1]) + (input[0] * input[1] * input[2])

    return Pair(paper, ribbon)
}