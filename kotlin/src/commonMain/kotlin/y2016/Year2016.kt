package y2016

fun run2016(day: Int): Pair<String, String>? {
    return when (day) {
        1 -> runY2016D1(Resources.readInputFile( 2016, 1))
        in 2..25 -> null
        else -> throw IllegalArgumentException("Invalid day $day for 2016")
    }
}