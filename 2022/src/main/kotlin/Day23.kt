fun Position2d.N() : Position2d = Position2d(x, y-1)
fun Position2d.S() : Position2d = Position2d(x, y+1)
fun Position2d.W() : Position2d = Position2d(x-1, y)
fun Position2d.E() : Position2d = Position2d(x+1, y)
fun Position2d.NE() : Position2d = Position2d(x+1, y-1)
fun Position2d.NW() : Position2d = Position2d(x-1, y-1)
fun Position2d.SE() : Position2d = Position2d(x+1, y+1)
fun Position2d.SW() : Position2d = Position2d(x-1, y+1)
fun Position2d.allNeighbours() : List<Position2d> =
    listOf(N(), S(), W(), E(), NE(), NW(), SE(), SW())

data class Elf(var pos: Position2d, var plan: Position2d?=null) {
    fun proposeMove(map: ElfMap) {
        if (map.isPosFree(pos.allNeighbours())) {
            plan = null
            return
        }
        for (dir in map.directions) {
            when (dir) {
                'N' -> if (map.isPosFree(listOf(pos.N(),pos.NE(),pos.NW()))) {
                    plan = pos.N()
                    return
                }
                'S' -> if (map.isPosFree(listOf(pos.S(),pos.SE(),pos.SW()))) {
                    plan = pos.S()
                    return
                }
                'W' -> if (map.isPosFree(listOf(pos.W(),pos.NW(),pos.SW()))) {
                    plan = pos.W()
                    return
                }
                'E' -> if (map.isPosFree(listOf(pos.E(),pos.NE(),pos.SE()))) {
                    plan = pos.E()
                    return
                }
            }
        }
    }
    fun move() {
        if (plan != null) {
            pos = plan!!
            plan = null
        }
    }
}


data class ElfMap(
    val elves: Set<Elf>,
    var round: Int = 0,
    var directions: MutableList<Char> = mutableListOf('N', 'S', 'W', 'E')) {
    fun isPosFree(l: List<Position2d>): Boolean =
        l.all {p -> p !in elves.map {it.pos}}
    fun nextDirection() {
        val tmp = directions.first()
        directions.removeAt(0)
        directions.add(tmp)
    }
    fun round(): Boolean {
        round++
        elves.forEach{ it.proposeMove(this) }

        val allProposedMoves = elves.mapNotNull { it.plan }
        if (allProposedMoves.isEmpty())
            // no more moves
            return false

        val stillMoving = elves.map { elf ->
            val targetMovers = allProposedMoves.count { it == elf.plan }
            if (targetMovers == 1) {
                elf.move()
                true
            }
            else {
                elf.plan = null
                false
            } // do not move, reset plan
        }
        // still moving
        nextDirection()
        return stillMoving.any()
    }
    fun boundingbox(border: Int): BoundingBox2d =
        BoundingBox2d(
    elves.minOf { it.pos.x } - border,
    elves.maxOf { it.pos.x } + border,
    elves.minOf { it.pos.y } - border,
    elves.maxOf { it.pos.y } + border,
        )

    fun countEmptyGround() : Int {
        var count = 0
        val bb = boundingbox(0)
        for (y in bb.minY..bb.maxY) {
            for (x in bb.minX..bb.maxX) {
                if (Position2d(x,y) !in elves.map { it.pos })
                    count++
            }
        }
        return count
    }
    fun prettyprint(): String = buildString {
        this.append("\n== End of Round $round ==\n")
        val bb = boundingbox(2)
        for (y in bb.minY .. bb.maxY) {
            for (x in bb.minX .. bb.maxX) {
                when {
                    Position2d(x,y) in elves.map {it.pos} -> this.append('#')
                    else -> this.append('.')
                }
            }
            this.append('\n')
        }
    }
    companion object {
        fun fromString(input: String): ElfMap {
            val positions = mutableSetOf<Elf>()
            input
                .split('\n')
                .filter { it.isNotEmpty() }
                .forEachIndexed { y, line ->
                    line.forEachIndexed { x, c ->
                        if (c == '#')
                            positions += Elf(Position2d(x,y))
                    }
                }
            return ElfMap(positions)
        }
    }
}

fun day23(test: Boolean = false) {
    val inputText = if (test)
        """
        ....#..
        ..###.#
        #...#.#
        .#...##
        #.###..
        ##.#.##
        .#..#..
        """.trimIndent()
    else
        InputTextDownloader().getText(2022, 23)

    val e = ElfMap.fromString(inputText)
    println(e.prettyprint())

    // Part A
    repeat(10) { e.round() }
    println(e.prettyprint())
    println(e.countEmptyGround())

    // Part B
    println()
    while (e.round()) {
        if (e.round % 10 == 0) println ("\rround ${e.round}")
    }
    e.prettyprint()
    println("no more move in round ${e.round}")


}
