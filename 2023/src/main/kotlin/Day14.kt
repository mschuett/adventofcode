
fun transpose(lines: List<CharArray>): List<CharArray> =
    (lines[0].indices).map { x ->
        (lines.indices).map { y ->
            lines[y][x]
        }.toCharArray()
    }

data class DishMap2D(var cols: List<CharArray>) {
    companion object Factory {
        fun fromString(inputText: String): DishMap2D {
            val inputLines = inputText.trim().split('\n')
            val cols = transpose(inputLines.map { it.toCharArray() })
            return DishMap2D(cols)
        }
    }
    fun prettyPrint() {
        var outstr: String = "DishMap2D(x=${cols.indices.last}, y=${cols[0].indices.last})\n"
//        outstr += "columns (=transposed):\n"
//        outstr += cols.joinToString("\n") { it.joinToString("") }
        outstr += "\nmap (='normal'):\n"
        outstr += transpose(cols).map { it.joinToString("") }.joinToString("\n")
        println(outstr)
    }

    fun toShortString() : String {
        return cols.joinToString("\n") { it.joinToString("") }
    }

    private fun rotateLeft() {
        cols = transpose(cols.reversed())
    }

    private fun rotateRight() {
        cols = transpose(cols).reversed()
    }

    fun calcWeight() : Int =
        transpose(cols)
            .map { it.joinToString("") }
            .reversed()
            .mapIndexed { i, line ->
                line.count { it == 'O' } * (i + 1)
            }.sum()

    fun tiltNorth() {
        for (col in cols) {
            var lastObstacle = -1
            col.indices.forEach { x ->
                when(col[x]) {
                    '#' -> {  // remember block
                        lastObstacle = x
                    }
                    'O' -> {  // roll left
                        col[x] = '.'
                        lastObstacle++
                        col[lastObstacle] = 'O'
                    }
                    '.' -> {}  // nothing
                    else -> TODO()
                }
            }
        }
    }

    fun cycle(): Int {
        // we do not want to tilt west/south/east but rotate the map instead...
        tiltNorth()
        rotateRight()
        tiltNorth()
        rotateRight()
        tiltNorth()
        rotateRight()
        tiltNorth()
        rotateRight()
        return calcWeight()
    }
}

fun day14(test: Boolean = true) {
    val inputText = if (test)
        """
        O....#....
        O.OO#....#
        .....##...
        OO.#O....O
        .O.....O#.
        O.#..O.#.#
        ..O..#O..O
        .......O..
        #....###..
        #OO..#....
        """.trimIndent()
    else
        InputTextDownloader().getText(2023, 14)

    // parsing

    // Part One
    val myMap = DishMap2D.fromString(inputText)
    myMap.tiltNorth()
    //myMap.prettyPrint()
    println("summarized: ${myMap.calcWeight()}")

    // Part Two
    val secondMap = DishMap2D.fromString(inputText)
    val totalCycles = 1000000000L
    val history = mutableListOf<Pair<Int,String>>(0 to "")  // use invalid element 0, to effectively start at 1
    (1..1000).forEach { i ->
        val shortString = secondMap.toShortString()
        val oldCycle = history.indexOfFirst { it.second == shortString }
        if (oldCycle != -1) {
            val periodLength = i - oldCycle
            println("found cycle from $oldCycle to $i -> length $periodLength")
            val periodTimes = ((totalCycles - oldCycle).div(periodLength))
            val resultCycle = totalCycles - (periodLength * periodTimes)
            println("same as in cycle $resultCycle -> weight ${history[resultCycle.toInt()].first}")
            return
        } else {
            val weight = secondMap.cycle()
            println("cycle $i --> weight $weight")
            history.add(weight to shortString)
        }
    }
}
