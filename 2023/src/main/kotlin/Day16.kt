
data class LaserMap2D(var lines: List<String>, val xMax: Int, val yMax: Int, val startBeam: Beam, var beams: MutableSet<Beam> = mutableSetOf(), val beamHistory: MutableSet<Beam> = mutableSetOf()) {
    companion object Factory {
        fun fromString(inputText: String, startBeam: Beam): LaserMap2D {
            val lines = inputText.trim().split('\n')
            return LaserMap2D(lines, lines[0].indices.last, lines.indices.last, startBeam=startBeam, beams=mutableSetOf(startBeam.copy()))
        }
    }
    fun prettyPrint() {
        var outstr: String = "${this.javaClass.name}(x=${xMax}, y=${yMax})\n"
        outstr += lines.joinToString("\n")
        outstr += "\nEnergized Fields (${beamHistory.map { it.pos }.distinct().count()})\n"
        (0..yMax).forEach { y ->
            (0..xMax).forEach { x ->
                outstr += if (Coord2D(x,y) in beamHistory.map { it.pos }) '#' else '.'
            }
            outstr += '\n'
        }
        println(outstr)
    }

    private fun getFieldAtPos(pos: Coord2D) = lines[pos.y][pos.x]

    fun beamWalk(): Int {
        var i = 0
        while (beams.isNotEmpty()) {
            beamWalkStep()
            i++
//            if (i.rem(10) == 0)
//                println("$i: ${beams.size} beams ${beamHistory.map { it.pos }.distinct().count()} energized")
        }

        // workaround: remove pre-starting pos:
        beamHistory.remove(startBeam)

        return beamHistory.map { it.pos }.distinct().count()
    }

    private fun beamWalkStep() {
        val newBeams = mutableSetOf<Beam>()
        beams.forEach { beamHistory.add(it.copy()) }

        for (beam in beams) {
            val newPos = beam.pos + beam.dir
            if (newPos.x < 0 || newPos.y < 0 || newPos.x > xMax || newPos.y > yMax) {
                continue
            }
            when (getFieldAtPos(newPos)) {
                '.' -> {}
                '\\' -> {
                    when (beam.dir) {
                        Direction2D.E -> beam.dir = Direction2D.S
                        Direction2D.W -> beam.dir = Direction2D.N
                        Direction2D.N -> beam.dir = Direction2D.W
                        Direction2D.S -> beam.dir = Direction2D.E
                    }
                }
                '/' -> {
                    when (beam.dir) {
                        Direction2D.E -> beam.dir = Direction2D.N
                        Direction2D.W -> beam.dir = Direction2D.S
                        Direction2D.N -> beam.dir = Direction2D.E
                        Direction2D.S -> beam.dir = Direction2D.W
                    }
                }
                '|' -> {
                    when (beam.dir) {
                        Direction2D.E, Direction2D.W -> {
                            beam.dir = Direction2D.N
                            val newBeam = Beam(newPos, Direction2D.S)
                            if (newBeam !in beamHistory)
                                newBeams.add(newBeam)
                        }
                        Direction2D.N, Direction2D.S -> {}
                    }
                }
                '-' -> {
                    when (beam.dir) {
                        Direction2D.E, Direction2D.W -> {}
                        Direction2D.N, Direction2D.S -> {
                            beam.dir = Direction2D.E
                            val newBeam = Beam(newPos, Direction2D.W)
                            if (newBeam !in beamHistory)
                                newBeams.add(newBeam)
                        }
                    }
                }
                else -> TODO()
            }
            beam.pos = newPos
            if (beam in beamHistory)
                continue
            else
                newBeams.add(beam)
        }
        beams = newBeams
    }
}

data class Beam(var pos: Coord2D, var dir: Direction2D)

fun day16(test: Boolean = true) {
    val inputText = if (test)
        """
        .|...\....
        |.-.\.....
        .....|-...
        ........|.
        ..........
        .........\
        ..../.\\..
        .-.-/..|..
        .|....-|.\
        ..//.|....
        """.trimIndent()
    else
        InputTextDownloader().getText(2023, 16)

    // Part One
    val map = LaserMap2D.fromString(inputText, Beam(Coord2D(-1,0), Direction2D.E))
    println(map.beamWalk())

    // Part Two
    val allStartBeams: MutableList<Beam> = mutableListOf()
    (0..map.xMax).forEach { x ->
        allStartBeams.add(Beam(Coord2D(x, -1), Direction2D.S))
    }
    (0..map.xMax).forEach { x ->
        allStartBeams.add(Beam(Coord2D(x, map.yMax+1), Direction2D.N))
    }
    (0..map.yMax).forEach { y ->
        allStartBeams.add(Beam(Coord2D(-1, y), Direction2D.E))
    }
    (0..map.yMax).forEach { y ->
        allStartBeams.add(Beam(Coord2D(map.xMax+1, y), Direction2D.W))
    }

    allStartBeams.map { start ->
        val energized = LaserMap2D.fromString(inputText, start).beamWalk()
        start to energized
    }.maxBy { it.second }.also(::println)
}
