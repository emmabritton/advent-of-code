package y2015

fun run2015(day: Int): Pair<String, String>? {
    return when (day) {
        1 -> runY2015D1(Resources.readInputFile( 2015, 1))
        2 -> runY2015D2(Resources.readInputFile( 2015, 2))
        in 3..25 -> null
        else -> throw IllegalArgumentException("Invalid day $day for 2015")
    }
}