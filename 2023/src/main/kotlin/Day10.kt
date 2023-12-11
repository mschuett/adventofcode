import kotlin.math.min

enum class Direction2D(val x: Int, val y: Int) {
    N(0,-1),
    E(1,0),
    S(0,1),
    W(-1,0),
}

data class Coord2D(val x: Int, val y: Int) {
    operator fun plus(dir: Direction2D): Coord2D =
        Coord2D(this.x + dir.x, this.y + dir.y)
}

data class PipeMap2D(var lines: List<String>, val xMax: Int, val yMax: Int) {
    companion object Factory {
        fun fromString(inputText: String): PipeMap2D {
            val lines = inputText.trim().split('\n')
            return PipeMap2D(lines, lines[0].indices.last, lines.indices.last)
        }
    }
    fun findStart() : Coord2D {
        (0..yMax).forEach { y ->
            (0..xMax).forEach { x ->
                if (lines[y][x] == 'S')
                    return Coord2D(x, y)
            }
        }
        throw IllegalStateException("map has no start")
    }

    private fun connectingNeighbours(src: Coord2D): List<Coord2D> =
        validPipeDirs[pipeAt(src)]!!
            .map { it to src + it }
            .filter { (dir, target) ->
                // check map bounds
                target.x in 0..xMax && target.y in 0..yMax
            }
            .filter { (dir, target) ->
                // check pipe connections
                when (dir) {
                    Direction2D.N -> Direction2D.S in validPipeDirs[pipeAt(target)]!!
                    Direction2D.S -> Direction2D.N in validPipeDirs[pipeAt(target)]!!
                    Direction2D.W -> Direction2D.E in validPipeDirs[pipeAt(target)]!!
                    Direction2D.E -> Direction2D.W in validPipeDirs[pipeAt(target)]!!
                }
            }.map { (dir, target) ->
                target
            }
    private fun pipeAt(coord: Coord2D): Char = lines[coord.y][coord.x]
    private val validPipeDirs: Map<Char, List<Direction2D>> = mapOf(
            '|' to listOf(Direction2D.N, Direction2D.S),
            '-' to listOf(Direction2D.E, Direction2D.W),
            'L' to listOf(Direction2D.N, Direction2D.E),
            'J' to listOf(Direction2D.N, Direction2D.W),
            '7' to listOf(Direction2D.S, Direction2D.W),
            'F' to listOf(Direction2D.S, Direction2D.E),
            'S' to listOf(Direction2D.S, Direction2D.E, Direction2D.N, Direction2D.W),
            '.' to listOf()
        )
    private var loopFields = setOf<Coord2D>()
    fun walk(start: Coord2D): Set<Coord2D> {
        println("walk at $start")
        var stepCount = 0
        var multiCursor = setOf(start)
        val seenFields = mutableMapOf<Coord2D, Int>(start to stepCount)
        do {
            stepCount++
            val newCursors = multiCursor
                .asSequence()
                .map { cursor ->
                    connectingNeighbours(cursor)
                }.flatten()
                .filterNot {
                    it in seenFields
                }
                .onEach {
                    seenFields[it] = stepCount
                }
                .toSet()
            multiCursor = newCursors
            // println("step ${stepCount}: newCursors $newCursors")
        } while (multiCursor.size > 1)
        println("found remote target at ${multiCursor.first()} with step count ${seenFields[multiCursor.first()]}")
        loopFields = seenFields.keys
        return seenFields.keys
    }
    fun prettyPrint() {
        var outstr: String = "Map2D(x=${xMax}, y=${yMax})\n"
        (0..yMax).forEach { y ->
            (0..xMax).forEach { x ->
                outstr += lines[y][x]
            }
            outstr += '\n'
        }

        println(outstr
            .replace('-', '─')
            .replace('|', '│')
            .replace('L', '╰')
            .replace('J', '╯')
            .replace('7', '╮')
            .replace('F', '╭')
        )
    }
    fun removeOtherPipelines() {
        val newLines = mutableListOf<String>()
        (0..yMax).forEach { y ->
            var newline = ""
            (0..xMax).forEach { x ->
                if (Coord2D(x, y) in loopFields || lines[y][x] == '.')
                    newline += lines[y][x]
                else
                    newline += '.'
            }
            newLines.add(newline)
        }
        lines = newLines
    }
    private fun isInsideField(field: Coord2D): Boolean {
        fun countCrossings(partialLine: String): Int {
            val startTile = '7'  // we cheat a little and manually insert the correct value here
            val pipesLeft = partialLine
                .replace(" ", "")
                .replace("-", "")
                .replace(".", "")
            println("          -> $pipesLeft")
            val pipeCounts = pipesLeft
                .replace('S', startTile)
                .toCharArray().toList().sorted()
                .groupingBy { it }
                .eachCount()
                .toMutableMap()
            println("          -> $pipeCounts")
            // ensure we get no nulls in lookup
            pipeCounts['|'] = pipeCounts['|'] ?: 0
            pipeCounts['F'] = pipeCounts['F'] ?: 0
            pipeCounts['7'] = pipeCounts['7'] ?: 0
            pipeCounts['J'] = pipeCounts['J'] ?: 0
            pipeCounts['L'] = pipeCounts['L'] ?: 0

            // some bends cancel each other
            val matchingParts1 = min(pipeCounts['F']!!, pipeCounts['7']!!)
            if (matchingParts1 > 0) {
                pipeCounts['F'] = pipeCounts['F']!! - matchingParts1
                pipeCounts['7'] = pipeCounts['7']!! - matchingParts1
            }
            val matchingParts2 = min(pipeCounts['L']!!, pipeCounts['J']!!)
            if (matchingParts2 > 0) {
                pipeCounts['L'] = pipeCounts['L']!! - matchingParts2
                pipeCounts['J'] = pipeCounts['J']!! - matchingParts2
            }
            println("          -> ${pipeCounts}")
            // count the remaining crossings
            val crossings = (pipeCounts['|']!!
                            + (pipeCounts['J']!! + pipeCounts['F']!!)/2
                            + (pipeCounts['L']!! + pipeCounts['7']!!)/2)
            return crossings
        }
        println("$field")
        val crossLeft  = countCrossings(lines[field.y].slice(0..field.x))
        val crossRight = countCrossings(lines[field.y].slice(field.x..xMax))
        assert(crossLeft.rem(2) == crossRight.rem(2))
        val result = crossLeft.rem(2) != 0
        println("          -> $crossLeft and $crossRight crossings => inside? $result")
        return result
    }
    fun markInsideOutsideGround() {
        val newLines = mutableListOf<String>()
        (0..yMax).forEach { y ->
            var newline = ""
            (0..xMax).forEach { x ->
                if (lines[y][x] == '.')
                    if (isInsideField(Coord2D(x, y)))
                        newline += '*'
                    else
                        newline += 'O'
                else
                    newline += lines[y][x]
            }
            newLines.add(newline)
        }
        lines = newLines
    }
}

fun day10(test: Boolean = true) {
    val inputText = if (test)
        """
        FF7FSF7F7F7F7F7F---7
        L|LJ||||||||||||F--J
        FL-7LJLJ||||||LJL-77
        F--JF--7||LJLJ7F7FJ-
        L---JF-JLJ.||-FJLJJ7
        |F|F-JF---7F7-L7L|7|
        |FFJF7L7F-JF7|JL---7
        7-L-JL7||F7|L7F-7F7|
        L.L7LFJ|||||FJL7||LJ
        L7JLJL-JLJLJL--JLJ.L
        """.trimIndent()
    else
        InputTextDownloader().getText(2023, 10)

    // parsing
    val myMap = PipeMap2D.fromString(inputText)
    val start = myMap.findStart()

    // Part One
    myMap.walk(start)

    // Part Two
    myMap.prettyPrint()
    // remove other pipeline fields, not part of the main loop, replaces them with ground
    myMap.removeOtherPipelines()
    // myMap.prettyPrint()
    // inside/outside check
    myMap.markInsideOutsideGround()
    myMap.prettyPrint()
    myMap.lines.sumOf {
        it.count { c -> c == '*' }
    }.also(::println)
}
