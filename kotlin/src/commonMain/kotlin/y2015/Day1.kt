package y2015

fun runY2015D1(input: String): Pair<String, String> {
    var floor = 0
    var basement: Int? = null

    input.onEachIndexed { index, char ->
        when (char) {
            '(' -> floor++
            ')' -> {
                floor--
                if (floor < 0 && basement == null) {
                    basement = index + 1
                }
            }
        }
    }

    return Pair(floor.toString(), basement.toString())
}