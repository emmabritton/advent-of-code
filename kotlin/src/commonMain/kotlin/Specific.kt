fun runSpecific(year: Int, day: Int) {
    val results = run(year, day);

    println("Year $year Day $day")

    if (results != null) {
        checkAnswer(results.first, year, day, 1)
        checkAnswer(results.second, year, day, 2)
    } else {
        println("Not implemented")
    }
}

fun checkAnswer(result: String, year: Int, day: Int, part: Int) {
    try {
        val answer = Resources.readAnswerFile(year, day, part)
        if (answer == null) {
            println("No answer provided for part $part, calculated was '$result'")
        } else {
            if (answer == result) {
                println("Provided answer and result match for part $part, both are $answer")
            } else {
                println("Provided answer and result mismatch for part $part, answer: '$answer' result: '$result'")
            }
        }
    } catch (e: Exception) {
        println("Unable to open answer for part $part: ${e::class.simpleName} ${e.message}")
    }
}