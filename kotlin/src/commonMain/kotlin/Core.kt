import y2015.run2015
import y2016.run2016

fun start(args: Array<String>) {
//    runSpecific(2015, 1)
    when (val mode = getMode(args)) {
        Mode.All -> runAll()
        is Mode.Specific -> runSpecific(mode.year, mode.day)
    }
}

sealed class Mode {
    object All : Mode()
    class Specific(val year: Int, val day: Int) : Mode()
}

fun getMode(args: Array<String>): Mode {
    if (args.size == 2) {
        val year = args[0].toIntOrNull()
        val day = args[1].toIntOrNull()

        return if (year != null && day != null) {
            if (year < 2015 || year > 2020 || day < 0 || day > 25) {
                throw IllegalArgumentException("Year must be in 2015..2020 and day must be in 1..25")
            } else {
                Mode.Specific(year, day)
            }
        } else {
            Mode.All
        }
    } else {
        return Mode.All
    }
}



fun run(year: Int, day: Int): Pair<String, String>? {
    return when (year) {
        2015 -> run2015(day)
        2016 -> run2016(day)
        in 2017..2020 -> null
        else -> throw IllegalArgumentException("Invalid year $year")
    }
}

