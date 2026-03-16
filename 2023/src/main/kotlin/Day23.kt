data class HikeMap(var lines: List<CharArray>, val start: Coord2D, val target: Coord2D) {
    companion object Factory {
        @ExperimentalStdlibApi
        fun fromString(inputText: String): HikeMap {
            val lines = inputText.trim()
                .split('\n')
                .map { it.toCharArray() }
            val startX = (0..<lines[0].size)
                .mapIndexed { i, x -> i to lines[0][x] }
                .filter { (i,c) -> c != '#'}
                .map { (i,c) -> i }
                .first()
            val targetX = (0..<lines[lines.size-1].size)
                .mapIndexed { i, x -> i to lines[lines.size-1][x] }
                .filter { (i,c) -> c != '#'}
                .map { (i,c) -> i }
                .first()
            val start = Coord2D(startX, 0)
            val target = Coord2D(targetX, lines.size-1)
            return HikeMap(lines, start, target)
        }
    }
    private val ySize: Int = lines.size
    private val xSize: Int = lines[0].size

    private fun getFieldAtPos(pos: Coord2D) = lines[pos.y][pos.x]

    fun prettyPrint() {
        var outstr: String = ""
        lines.forEachIndexed { y, line ->
            line.forEachIndexed { x, c ->
                if (start == Coord2D(x, y))
                    outstr += 'S'
                else if (target == Coord2D(x, y))
                    outstr += 'T'
                else
                    outstr += lines[y][x]
            }
            outstr += '\n'
        }
        outstr += "${this.javaClass.name}(x=${xSize}, y=${ySize})\n"
        println(outstr)
    }

    @ExperimentalStdlibApi
    fun broadWalk(icySlopes: Boolean = true) {
        val initWalk = mutableListOf<Coord2D>(start, start+Direction2D.S)
        val walks: MutableList<MutableList<Coord2D>> = mutableListOf(initWalk)
        do {
            val newWalks: MutableList<MutableList<Coord2D>> = mutableListOf()
            val stuckWalks: MutableList<MutableList<Coord2D>> = mutableListOf()
            for (walk in walks) {
                val pos = walk.last()
                if (pos == target)
                    continue
                val newPos = listOf(
                    Direction2D.N,
                    Direction2D.S,
                    Direction2D.E,
                    Direction2D.W,
                )
                .asSequence()
                .map { dir ->
                    dir to pos + dir
                }.filter { (dir, pos) ->
                    getFieldAtPos(pos) != '#'
                }.filter { (dir, pos) ->
                    (!icySlopes) || (
                            (getFieldAtPos(pos) != '^' || dir == Direction2D.N) &&
                            (getFieldAtPos(pos) != 'v' || dir == Direction2D.S) &&
                            (getFieldAtPos(pos) != '<' || dir == Direction2D.W) &&
                            (getFieldAtPos(pos) != '>' || dir == Direction2D.E)
                            )
                }.filter { (dir, pos) ->
                    pos !in walk
                }.map { (dir, pos) ->
                    pos
                }.toList()

                if (newPos.isEmpty()) {
                    stuckWalks.add(walk)
                } else if (newPos.size == 1) {
                    walk.add(newPos[0])
                } else if (newPos.size == 2) {
                    val walk1 = walk.toList().toMutableList()
                    walk.add(newPos[0])
                    walk1.add(newPos[1])
                    newWalks.add(walk1)
                } else if (newPos.size == 3) {
                    val walk1 = walk.toList().toMutableList()
                    val walk2 = walk.toList().toMutableList()
                    walk.add(newPos[0])
                    walk1.add(newPos[1])
                    walk2.add(newPos[2])
                    newWalks.add(walk1)
                    newWalks.add(walk2)
                }
            }
            walks.removeAll(stuckWalks)
            walks.addAll(newWalks)
        } while (walks.any { it.last() != target })


        println("left with ${walks.size} walks")
        val maxSteps = walks.maxByOrNull { it.size }!!.size
            .also { steps -> println("longest one with ${steps-1} steps") }
    }

//    @ExperimentalStdlibApi
//    fun deepWalk(icySlopes: Boolean = true) {
//        fun getNextStep(pos: Coord2D, walk: MutableList<Pair<Coord2D,Int>>): List<Coord2D> =
//            listOf(
//                    Direction2D.N,
//                    Direction2D.S,
//                    Direction2D.E,
//                    Direction2D.W,
//                )
//                // .asSequence()
//                .map { dir ->
//                    dir to pos + dir
//                }.filter { (dir, pos) ->
//                    pos.x in 0..<xSize && pos.y in 0..<ySize
//                }.filter { (dir, pos) ->
//                    getFieldAtPos(pos) != '#'
//                }.filter { (dir, pos) ->
//                    (!icySlopes) || (
//                            (getFieldAtPos(pos) != '^' || dir == Direction2D.N) &&
//                                    (getFieldAtPos(pos) != 'v' || dir == Direction2D.S) &&
//                                    (getFieldAtPos(pos) != '<' || dir == Direction2D.W) &&
//                                    (getFieldAtPos(pos) != '>' || dir == Direction2D.E)
//                            )
//                }.filter { (dir, pos) ->
//                    pos !in walk.map { it.first }
//                }.map { (dir, pos) ->
//                    pos
//                }.toList()
//
//        // save junction positions and distances between
//        val initWalk = mutableListOf<Pair<Coord2D,Int>>(start to 0)
//        val walks: MutableList<MutableList<Pair<Coord2D,Int>>> = mutableListOf(initWalk)
//
//        do {
//            for (walk in walks) {
//                var pos = walk.last().first
//                if (pos == target)
//                    continue
//
//                val newWalks: MutableList<MutableList<Pair<Coord2D,Int>>> = mutableListOf()
//                val stuckWalks: MutableList<MutableList<Pair<Coord2D,Int>>> = mutableListOf()
//                var steps = 0
//                var nextStepPos: List<Coord2D>
//                do {
//                    nextStepPos = getNextStep(pos, walk)
//                    if (nextStepPos.size == 1) {
//                        steps++
//                        pos = nextStepPos.first()
//                    }
//                    else
//                        // otherwise pos is the "old pos" and junction field coord
//                        break
//                } while (true)
//
//                if (nextStepPos.isEmpty()) {
//                    stuckWalks.add(walk)
//                } else {
//                    walk.add(pos to steps) // save the junction before the branch
//                }
//
//                // for all branches: walk on
//                nextStepPos.forEach { nextPos ->
//                    newWalks.add(walk)
//                }
//
//                if (nextStepPos.size == 2) {
//                    val walk1 = walk.toList().toMutableList()
//                    walk.add(pos to steps)
//
//                    newWalks.add()
//                    // save junction point
//                    walk.add(pos to steps-1)
//
//
//                } else if (nextStepPos.size == 3) {
//                    val walk1 = walk.toList().toMutableList()
//                    val walk2 = walk.toList().toMutableList()
//                    walk.add(nextStepPos[0])
//                    walk1.add(nextStepPos[1])
//                    walk2.add(nextStepPos[2])
//                    newWalks.add(walk1)
//                    newWalks.add(walk2)
//                } else
//                    TODO()
//            }
//            walks.removeAll(stuckWalks)
//            walks.addAll(newWalks)
//        } while (walks.any { it.last() != target })
//
//
//        println("left with ${walks.size} walks")
//        val maxSteps = walks.maxByOrNull { it.size }!!.size
//            .also { steps -> println("longest one with ${steps-1} steps") }
//    }
}

@OptIn(ExperimentalStdlibApi::class)
fun day23(test: Boolean = false) {
    val inputText = if (test)
        """
        #.#####################
        #.......#########...###
        #######.#########.#.###
        ###.....#.>.>.###.#.###
        ###v#####.#v#.###.#.###
        ###.>...#.#.#.....#...#
        ###v###.#.#.#########.#
        ###...#.#.#.......#...#
        #####.#.#.#######.#.###
        #.....#.#.#.......#...#
        #.#####.#.#.#########v#
        #.#...#...#...###...>.#
        #.#.#v#######v###.###v#
        #...#.>.#...>.>.#.###.#
        #####v#.#.###v#.#.###.#
        #.....#...#...#.#.#...#
        #.#########.###.#.#.###
        #...###...#...#...#.###
        ###.###.#.###v#####v###
        #...#...#.#.>.>.#.>.###
        #.###.###.#.###.#.#v###
        #.....###...###...#...#
        #####################.#
        """.trimIndent()
    else
        InputTextDownloader().getText(2023, 23)

    // parsing
    val map = HikeMap.fromString(inputText)
    map.prettyPrint()

    // Part One
    map.broadWalk()

    // Part Two
    map.broadWalk(false)

}
