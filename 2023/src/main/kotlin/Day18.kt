

data class Digger(var pos: Coord2D, val steps: List<Pair<Direction2D, Int>>, val trench: MutableSet<Coord2D> = mutableSetOf()) {
    fun dig() {
        for (step in steps) {
            repeat(step.second) {
                trench += pos
                pos += step.first
            }
        }
    }

    fun getInnerFields(): Set<Coord2D> {
        val minX = trench.minOf { it.x }
        val maxX = trench.maxOf { it.x }
        val minY = trench.minOf { it.y }
        val maxY = trench.maxOf { it.y }
        var i = 0
        while (Coord2D(minX + i, minY + i) !in trench) { i++ }
        i++
        val center = Coord2D(minX + i, minY + i)
        assert(center !in trench)

        val result: MutableSet<Coord2D> = mutableSetOf(center)
        do {
            val newFields = result
                .flatMap {
                    listOf(
                        it + Direction2D.N,
                        it + Direction2D.S,
                        it + Direction2D.E,
                        it + Direction2D.W
                    )
                }.filterNot {
                    it in trench || it in result
                }.onEach {
                    assert(it.x in (minX + 1) until maxX)
                    assert(it.y in (minY + 1) until maxY)
                }
            result += newFields
        } while (newFields.isNotEmpty())
        return result
    }

    fun printMap(innerFields: Set<Coord2D> = setOf()): String {
        var outstring = ""
        val minX = trench.minOf { it.x }
        val maxX = trench.maxOf { it.x }
        val minY = trench.minOf { it.y }
        val maxY = trench.maxOf { it.y }

        (minY..maxY).forEach { y ->
            (minX..maxX).forEach { x ->
                outstring += if (Coord2D(x,y) in trench)
                    '#'
                else if (Coord2D(x,y) in innerFields)
                    'i'
                else
                    '.'

            }
            outstring += '\n'
        }
        println(outstring)
        return outstring
    }
}

fun day18(test: Boolean = true) {
    val inputText = if (test)
        """
        R 6 (#70c710)
        D 5 (#0dc571)
        L 2 (#5713f0)
        D 2 (#d2c081)
        R 2 (#59c680)
        D 2 (#411b91)
        L 5 (#8ceee2)
        U 2 (#caa173)
        L 1 (#1b58a2)
        U 2 (#caa171)
        R 2 (#7807d2)
        U 3 (#a77fa3)
        L 2 (#015232)
        U 2 (#7a21e3)
        """.trimIndent()
    else
        InputTextDownloader().getText(2023, 18)

    // Part One
    val steps = inputText.trim()
        .split('\n')
        .map { line ->
            val (dirChar, lenChars, colChars) = line.split(' ')
            val dir = when(dirChar) {
                "U" -> Direction2D.N
                "D" -> Direction2D.S
                "R" -> Direction2D.E
                "L" -> Direction2D.W
                else -> TODO()
            }
            dir to lenChars.toInt()
        }
    val digger = Digger(Coord2D(0,0), steps)
    digger.dig()
    val trenchFields = digger.trench.size
    val insideFields = digger.getInnerFields()
    val insideFieldCount = insideFields.size
    digger.printMap(insideFields)
    println("${trenchFields} + ${insideFieldCount} = ${trenchFields + insideFieldCount}")

    // Part Two
    val hexSteps = inputText.trim()
        .split('\n')
        .map { line ->
            var (_, _, hexChars) = line.split(' ')
            hexChars = hexChars.trim('(', '#', ')')
            val dir = when(hexChars.last()) {
                '3' -> Direction2D.N
                '1' -> Direction2D.S
                '0' -> Direction2D.E
                '2' -> Direction2D.W
                else -> TODO()
            }
            dir to hexChars.dropLast(1).toInt(16)
        }
    val digger2 = Digger(Coord2D(0,0), hexSteps)
    digger2.dig()
    val trenchFields2 = digger2.trench.size
    val insideFields2 = digger2.getInnerFields()
    val insideFieldCount2 = insideFields.size
    digger2.printMap(insideFields2)
    println("${trenchFields2} + ${insideFieldCount2} = ${trenchFields2 + insideFieldCount2}")

}
