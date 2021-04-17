import java.io.File

actual object Resources {
    private val answerDir by lazy { getResourceDir("answers") }
    private val inputDir by lazy { getResourceDir("inputs") }

    actual fun readAnswerFile(year: Int, day: Int, part: Int): String? {
        val path = File(answerDir, "$year/day${day}p$part")
        return if (path.exists()) {
            path.readText().trim()
        } else {
            null
        }
    }

    actual fun readInputFile(year: Int, day: Int): String {
        val path = File(inputDir, "$year/day${day}")
        return if (path.exists()) {
            path.readText().trim()
        } else {
            throw IllegalArgumentException("No input file found for $year/$day")
        }
    }

    private fun getResourceDir(name: String): File {
        val workingDir = File(System.getProperty("user.dir"))
        return File(workingDir.parentFile, "resources/$name")
    }
}