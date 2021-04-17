fun runAll() {
    for (year in 2015..2020) {
        println("\n\n$year\n")
        (1..25)
            .map { runDay(year, it) }
            .forEach { (day, result) ->
                val padded = day.toString().padStart(2, ' ')
                println("$padded    $result")
            }
    }
}

fun runDay(year: Int, day: Int): Pair<Int, String> {
    return try {
        val results = run(year, day)
        if (results != null) {
            val answer1 = getAnswer(results.first, year, day, 1)
            val answer2 = getAnswer(results.second, year, day, 2)
            Pair(day, "[$answer1] [$answer2]")
        } else {
            Pair(day, "Not implemented")
        }
    } catch (e: Exception) {
        Pair(day, "Error running")
    }
}

fun getAnswer(result: String, year: Int, day: Int, part: Int): String {
    return try {
        val answer = Resources.readAnswerFile(year, day, part)
        if (answer != null) {
            if (result == answer) {
                "✓"
            } else {
                "✖"
            }
        } else {
            "?"
        }
    } catch (e: Exception) {
        "⚠"
    }
}