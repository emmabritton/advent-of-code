import kotlinx.cinterop.ByteVar
import kotlinx.cinterop.allocArray
import kotlinx.cinterop.memScoped
import kotlinx.cinterop.toKString
import platform.posix.*

actual object Resources {
    private const val READ_MODE = "r"
    private const val ERROR = -1

    private val answerDir by lazy { getResourceDir("answers") }
    private val inputDir by lazy { getResourceDir("inputs") }

    actual fun readAnswerFile(year: Int, day: Int, part: Int): String? {
        val path = "$answerDir/$year/day${day}p$part"
        return if (doesFileExist(path)) {
            readAllText(path).trim()
        } else {
            null
        }
    }

    actual fun readInputFile(year: Int, day: Int): String {
        val path = "$inputDir/$year/day$day"
        if (doesFileExist(path)) {
            return readAllText(path)
        } else {
            throw IllegalArgumentException("No input file found for $year/$day")
        }
    }

    private fun getWorkingDir(): String {
        memScoped {
            val pathLength: ULong = PATH_MAX.toULong() + 1.toULong()
            val buffer = allocArray<ByteVar>(pathLength.toInt())
            return getcwd(buffer, pathLength)?.toKString() ?: throw IllegalStateException("Couldn't get working directory")
        }
    }

    private fun getResourceDir(name: String): String {
        val workingDir = getWorkingDir()
        val idx = workingDir.lastIndexOf('/')
        if (idx == -1) {
            workingDir.lastIndexOf('\\')
        }
        if (idx == -1) {
            throw IllegalStateException("Program should be run from 'kotlin-native' dir")
        }
        val parent =workingDir.subSequence(0, idx)
        return "$parent/resources/$name"
    }

    private fun doesFileExist(path: String): Boolean {
        return access(path, F_OK) != ERROR;
    }

    private fun readAllText(filePath: String): String {
        val returnBuffer = StringBuilder()
        val file = fopen(filePath, READ_MODE) ?: throw IllegalArgumentException("Cannot open input file $filePath")

        try {
            memScoped {
                val readBufferLength = 64 * 1024
                val buffer = allocArray<ByteVar>(readBufferLength)
                var line = fgets(buffer, readBufferLength, file)?.toKString()
                while (line != null) {
                    returnBuffer.append(line)
                    line = fgets(buffer, readBufferLength, file)?.toKString()
                }
            }
        } finally {
            fclose(file)
        }

        return returnBuffer.toString()
    }
}