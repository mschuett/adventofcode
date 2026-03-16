
data class GardenMap2D(
    var lines: List<CharArray>,
    var positions: Set<Coord2D> = setOf(),
    var steps: Int = 0) {
    companion object Factory {
        fun fromString(inputText: String): GardenMap2D {
            val positions = mutableSetOf<Coord2D>()
            val lines = inputText.trim()
                .split('\n')
                .map { it.toCharArray() }
            lines.forEachIndexed { y, line ->
                line.forEachIndexed { x, c ->
                    if (c == 'S') positions.add(Coord2D(x, y))
                }
            }
            positions.forEach { (x,y) ->
                lines[y][x] = '.'
            }
            return GardenMap2D(lines, positions, 0)
        }
    }
    private val ySize: Int = lines.size
    private val xSize: Int = lines[0].size

    private fun getFieldAtPos(pos: Coord2D) = lines[pos.y.mod(ySize)][pos.x.mod(xSize)]

    fun prettyPrint() {
        var outstr: String = ""
        lines.forEachIndexed { y, line ->
            line.forEachIndexed { x, c ->
                if (positions.contains(Coord2D(x, y)))
                    outstr += 'O'
                else
                    outstr += lines[y][x]
            }
            outstr += '\n'
        }
        outstr += "${this.javaClass.name}(x=${xSize}, y=${ySize}, steps=${steps}, #positions=${positions.size})\n"
        println(outstr)
    }

    fun walkStep() {
        positions = positions.flatMap {
            listOf(
                it + Direction2D.N,
                it + Direction2D.S,
                it + Direction2D.E,
                it + Direction2D.W,
            )
        }.filterNot {
            getFieldAtPos(it) == '#'
        }.toSet()
        steps++
    }
    fun walk(steps: Int) = repeat(steps) { walkStep() }
}

fun day21(test: Boolean = true) {
    val inputText = if (test)
        """
        ...........
        .....###.#.
        .###.##..#.
        ..#.#...#..
        ....#.#....
        .##..S####.
        .##..#...#.
        .......##..
        .##.#.####.
        .##..##.##.
        ...........
        """.trimIndent()
    else
        InputTextDownloader().getText(2023, 21)

    // parsing

    // Part One
    val map = GardenMap2D.fromString(inputText)
    map.walk(if (test) 6 else 64)
    map.prettyPrint()

    // Part Two
    // of course this does not work as is but needs a more efficient approach...
    val infMap = GardenMap2D.fromString(inputText)
    if (test) {
        infMap.walk(6)
        println("${infMap.steps} steps -> ${infMap.positions.size}")
        infMap.walk(4)
        println("${infMap.steps} steps -> ${infMap.positions.size}")
        infMap.walk(40)
        println("${infMap.steps} steps -> ${infMap.positions.size}")
        infMap.walk(50)
        println("${infMap.steps} steps -> ${infMap.positions.size}")
        infMap.walk(400)
        println("${infMap.steps} steps -> ${infMap.positions.size}")
        infMap.walk(500)
        println("${infMap.steps} steps -> ${infMap.positions.size}")
        infMap.walk(4000)
        println("${infMap.steps} steps -> ${infMap.positions.size}")
    } else {
        infMap.walk(26501365)
        println("${infMap.steps} steps -> ${infMap.positions.size}")
    }
}
