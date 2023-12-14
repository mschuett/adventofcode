

data class MirrorMap2D(var lines: List<String>, val xMax: Int, val yMax: Int) {
    private val transLines: List<String> = transpose()
    private fun transpose(): List<String> =
        (0..xMax).map { x ->
            (0..yMax).map { y ->
                lines[y][x]
            }.joinToString("")
        }

    companion object Factory {
        fun fromString(inputText: String): MirrorMap2D {
            val lines = inputText.trim().split('\n')
            return MirrorMap2D(lines, lines[0].indices.last, lines.indices.last)
        }
    }
    fun prettyPrint() {
        var outstr: String = "MirrorMap2D(x=${xMax}, y=${yMax})\n"
        outstr += lines.joinToString("\n")
        println(outstr)
    }

    private fun diffString(a: String, b: String): Int =
        (a.toCharArray() zip b.toCharArray())
            .count { (a, b) -> a != b }

    private fun isMirroredAfterRow(row: Int, smudge: Int = 0): Boolean {
        val rowsBefore = row downTo 0
        val rowsAfter  = (row+1)..yMax
        return (rowsBefore zip rowsAfter)
            .sumOf { (before, after) ->
                diffString(lines[before], lines[after])
            } == smudge
    }

    private fun isMirroredAfterColumn(column: Int, smudge: Int = 0): Boolean {
        val colsBefore = column downTo 0
        val colsAfter  = (column+1)..xMax
        return (colsBefore zip colsAfter)
            .sumOf { (before, after) ->
                diffString(transLines[before], transLines[after])
            } == smudge
    }

    fun findMirrorRows(smudge: Int = 0): Int? =
        (0..yMax-1).find {
            isMirroredAfterRow(it, smudge)
        }?.plus(1)  // because task counts from line 1

    fun findMirrorColumns(smudge: Int = 0): Int? =
        (0..xMax-1).find {
            isMirroredAfterColumn(it, smudge)
        }?.plus(1)  // because task counts from line 1
}

fun day13(test: Boolean = true) {
    val inputText = if (test)
        """
        #.##..##.
        ..#.##.#.
        ##......#
        ##......#
        ..#.##.#.
        ..##..##.
        #.#.##.#.
        
        #...##..#
        #....#..#
        ..##..###
        #####.##.
        #####.##.
        ..##..###
        #....#..#
        """.trimIndent()
    else
        InputTextDownloader().getText(2023, 13)

    // parsing
    val allMaps = inputText.trim().split("\n\n")
        .map {
            MirrorMap2D.fromString(it)
        }

    // Part One
    val rowSum = allMaps.mapNotNull {
        it.findMirrorRows()
    }.also(::println).sum()
    val colSum = allMaps.mapNotNull {
        it.findMirrorColumns()
    }.also(::println).sum()
    println("summarized: ${100*rowSum + colSum}")

    // Part Two
    val rowSum2 = allMaps.mapNotNull {
        it.findMirrorRows(1)
    }.also(::println).sum()
    val colSum2 = allMaps.mapNotNull {
        it.findMirrorColumns(1)
    }.also(::println).sum()
    println("summarized: ${100*rowSum2 + colSum2}")


}
