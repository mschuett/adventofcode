fun Position2d.allManhattanNeighbours() : List<Position2d> =
    listOf(this, N(), S(), W(), E())

data class Blizzard(var pos: Position2d, val dir: Direction) {
    fun move(map: BlizzardMap) {
        when (dir) {
            Direction.E -> {
                pos.x++
                if (pos.x > map.bb.maxX-1)
                    pos.x = map.bb.minX+1
            }
            Direction.W -> {
                pos.x--
                if (pos.x < map.bb.minX+1)
                    pos.x = map.bb.maxX-1
            }
            Direction.N -> {
                pos.y--
                if (pos.y < map.bb.minY+1)
                    pos.y = map.bb.maxY-1
            }
            Direction.S -> {
                pos.y++
                if (pos.y > map.bb.maxY-1)
                    pos.y = map.bb.minY+1
            }
        }
    }
}

// approach: updating the BlizzardMap is expensive, so we run only one simulation of it;
// inside it we keep many ElfWalkers for their parallel BFS toward the exit
data class ElfWalker(var pos: Position2d) {
    fun step(map: BlizzardMap): List<ElfWalker> {
        val options = pos.allManhattanNeighbours()
            .filter { it in map.bb
                    && it !in map.walls
                    && it !in map.blizzardPositions
            }

        return options.map {
            copy(pos = it)
        }
    }
}

data class BlizzardMap(
    val walls: Set<Position2d>,
    var blizzards: Set<Blizzard>,
    val bb: BoundingBox2d = BoundingBox2d(
        walls.minOf {it.x},
        walls.maxOf {it.x},
        walls.minOf {it.y},
        walls.maxOf {it.y},
    ),
    var minute: Int = 0,
    var start: Position2d = Position2d(1,0),
    var target: Position2d = Position2d(bb.maxX-1, bb.maxY),
    var elves: List<ElfWalker> = listOf(ElfWalker(start)),
) {

    var blizzardPositions: Set<Position2d> = allBlizzardPositions()  // for performance optimization
    private fun allBlizzardPositions() : Set<Position2d> = blizzards.map { it.pos }.toSet()

    fun step(): Boolean {
        minute++
        blizzards.forEach { it.move(this) }
        blizzardPositions = allBlizzardPositions()

        elves = elves.flatMap {
            it.step(this)
        }.distinct()

        require(elves.all { it.pos !in blizzardPositions })
        require(elves.all { it.pos !in walls })
        return target in elves.map { it.pos }
    }

    fun prettyprint(): String = buildString {
        this.append("\n== End of Round $minute ==\n")
        this.append(blizzards)
        this.append('\n')
        this.append(elves)
        this.append('\n')
        for (y in bb.minY .. bb.maxY) {
            for (x in bb.minX .. bb.maxX) {
                val pos = Position2d(x,y)
                when (pos) {
                    in elves.map{it.pos} -> this.append('E')
                    in walls             -> this.append('#')
                    in blizzardPositions -> {
                        val b = blizzards.filter { it.pos == pos }
                        val count = b.count()
                        if (count > 9)
                            this.append('a' + count-10)
                        else if (count > 1)
                            this.append(count)
                        else
                            this.append(b.single().dir)
                    }
                    else -> this.append('.')
                }
            }
            this.append('\n')
        }
    }
    companion object {
        fun fromString(input: String): BlizzardMap {
            val positions = mutableSetOf<Blizzard>()
            val walls = mutableSetOf<Position2d>()
            input
                .split('\n')
                .filter { it.isNotEmpty() }
                .forEachIndexed { y, line ->
                    line.forEachIndexed { x, c ->
                        when(c) {
                            '#' -> walls += Position2d(x,y)
                            '<', '>', 'v', '^' -> positions += Blizzard(Position2d(x,y), Direction.fromChar(c))
                            '.' -> {}
                            else -> throw IllegalStateException("unexpected input char")
                        }
                    }
                }
            return BlizzardMap(walls, positions)
        }
    }
}

fun day24(test: Boolean = false) {
    val inputText = if (test)
        """
        #.######
        #>>.<^<#
        #.<..<<#
        #>v.><>#
        #<^v^^>#
        ######.#
        """.trimIndent()
    else
        InputTextDownloader().getText(2022, 24)

    val b = BlizzardMap.fromString(inputText)
    println(b)
    println(b.prettyprint())

    // Part A
    while (!b.step()) {}
    println(b.prettyprint())
    println("${b.minute} minutes to reach the goal")

    // Part B
    val target = b.target
    val start = b.start
    b.target = start
    b.start = target
    b.elves = listOf(ElfWalker(b.start))
    while (!b.step()) {}
    println(b.prettyprint())
    println("${b.minute} minutes to reach the start again")

    b.target = target
    b.start = start
    b.elves = listOf(ElfWalker(b.start))
    while (!b.step()) {}
    println(b.prettyprint())
    println("${b.minute} minutes to reach the target again")
}
