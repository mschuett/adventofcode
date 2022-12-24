data class MonkeyMap(val walkable: Set<Position2d>, val walls: Set<Position2d>) {
    fun startPos(): Position2d =
        walkable
            .filter {
                it.y == walkable.minOf { it.y }
            }
            .minBy {
                it.x
            }

    companion object {
        fun fromString(input: String): MonkeyMap {
            val walk = mutableSetOf<Position2d>()
            val wall = mutableSetOf<Position2d>()
            input
                .split('\n')
                .withIndex()
                .forEach { (y, line) ->
                    line
                        .withIndex()
                        .forEach { (x, c) ->
                            when (c) {
                                ' ' -> {} // nothing
                                '.' -> walk += Position2d(x+1,y+1)
                                '#' -> wall += Position2d(x+1,y+1)
                                else -> throw IllegalStateException("unexpected char")
                            }
                        }
                }
            return MonkeyMap(walk, wall)
        }
    }
}

enum class Direction (val i: Int, val c: Char){
    E(0, '>'),
    S(1, 'v'),
    W(2, '<'),
    N(3, '^');
    fun turn(c: Char): Direction =
        when (c) {
            'R' -> Direction.fromInt((i + 1).mod(Direction.values().size))
            'L' -> Direction.fromInt((i - 1).mod(Direction.values().size))
            else -> throw IllegalStateException("unexpected char")
        }
    override fun toString(): String = c.toString()
    companion object {
        fun fromInt(i: Int): Direction =
            Direction
                .values()
                .find {
                    i == it.i
                }!!
        fun fromChar(c: Char): Direction =
            Direction
                .values()
                .find {
                    c == it.c
                }!!
    }
}

fun Position2d.inDir(d: Direction): Position2d =
    when (d) {
        Direction.E -> copy(x = x+1)
        Direction.W -> copy(x = x-1)
        Direction.S -> copy(y = y+1)
        Direction.N -> copy(y = y-1)
    }

data class Actor(val map: MonkeyMap, val directions: Iterable<PathDirection>, var testMode: Boolean = true) {
    var pos: Position2d = map.startPos()
    var dir: Direction = Direction.E
    var trace: MutableList<Position2d> = mutableListOf(pos)
    var interactive = false
    var partB = false

    fun turn(c: Char) {
        dir = dir.turn(c)
    }
    fun walkForward() {
        if (interactive ) {
            println("\u001Bc pos: ${pos.x},${pos.y} dir: $dir")
            println(prettyPrint())
            // readln()
        }
        val newPos = pos.inDir(dir)
        if (newPos in map.walls) {
            // nothing
            return
        } else if (newPos in map.walkable) {
            pos = newPos
            trace += pos
            return
        }
        // need to wrap around
        if (partB) {
            val (wrappedPos, wrappedDir) = getWrappedPosCube()
            if (wrappedPos !in map.walls) {
                pos = wrappedPos
                dir = wrappedDir
            }
        } else {
            val wrappedPos = getWrappedPosSimple(map.walkable + map.walls)
            if (wrappedPos !in map.walls)
                pos = wrappedPos
        }
        trace += pos
    }
    private fun getWrappedPosSimple(tiles: Set<Position2d>) = when (dir) {
        Direction.S -> tiles
            .filter { it.x == pos.x }
            .minBy { it.y }

        Direction.N -> tiles
            .filter { it.x == pos.x }
            .maxBy { it.y }

        Direction.E -> tiles
            .filter { it.y == pos.y }
            .minBy { it.x }

        Direction.W -> tiles
            .filter { it.y == pos.y }
            .maxBy { it.x }
    }
    // arbitrary and hard-coded numbering of the six surfaces
    // for test like in the description, for real data like this:
    //     13
    //     2
    //    46
    //    5

    private val sl = if (testMode) 4 else 50  // side length
    private fun cubeSide(pos: Position2d): Int {
        return if (testMode)
                 if (pos.x in  9..12 && pos.y in  1.. 4) 1
            else if (pos.x in  1.. 4 && pos.y in  5.. 8) 2
            else if (pos.x in  5.. 8 && pos.y in  5.. 8) 3
            else if (pos.x in  9..12 && pos.y in  5.. 8) 4
            else if (pos.x in  9..12 && pos.y in  9..12) 5
            else if (pos.x in 13..16 && pos.y in  9..12) 6
            else throw IllegalStateException("invalid pos $pos")
        else
                 if (pos.x in   sl+1..2*sl && pos.y in      1.. sl)   1
            else if (pos.x in   sl+1..2*sl && pos.y in   sl+1.. 2*sl) 2
            else if (pos.x in 2*sl+1..3*sl && pos.y in      1.. sl)   3
            else if (pos.x in      1..  sl && pos.y in 2*sl+1.. 3*sl) 4
            else if (pos.x in      1..  sl && pos.y in 3*sl+1.. 4*sl) 5
            else if (pos.x in   sl+1..2*sl && pos.y in 2*sl+1.. 3*sl) 6
            else throw IllegalStateException("invalid pos $pos")

    }

    private fun getWrappedPosCube(): Pair<Position2d, Direction> {
        fun offsetInverse(x: Int, offDest: Int, offSrc: Int): Int = offDest*sl + (x - offSrc*sl)

        val response = if (testMode)
            when(cubeSide(pos)) {
            1 ->
                when (dir) {
                    Direction.S -> throw IllegalStateException("unexpected wrap")
                    Direction.N ->  // goto 2
                        Position2d(1*sl - (pos.x - 2*sl -1), 1*sl + 1) to Direction.S
                    Direction.E ->  // goto 6
                        Position2d(4*sl - 1, 4*sl - pos.y) to Direction.W
                    Direction.W ->  // goto 3
                        Position2d(1*sl + pos.y, 1*sl + 1) to Direction.S
                }

            2 ->
                when (dir) {
                    Direction.S -> throw IllegalStateException("unexpected wrap")
                    Direction.N -> throw IllegalStateException("unexpected wrap")
                    Direction.E ->  // goto 6
                        Position2d(4*sl - (pos.y-sl-1), 3*sl + 1) to Direction.N
                    Direction.W -> throw IllegalStateException("unexpected wrap")
                }
            3 ->
                when (dir) {
                    Direction.S ->  // goto 5
                        Position2d(2*sl + 1, 2*sl + (pos.x-sl)) to Direction.E
                    Direction.N ->  // goto 1
                        Position2d(2*sl + 1, 0*sl + (pos.x-sl)) to Direction.E
                    Direction.E -> throw IllegalStateException("unexpected wrap")
                    Direction.W -> throw IllegalStateException("unexpected wrap")
                }
            4 ->
                when (dir) {
                    Direction.S -> throw IllegalStateException("unexpected wrap")
                    Direction.N -> throw IllegalStateException("unexpected wrap")
                    Direction.E ->  // goto 6
                        Position2d(4*sl - (pos.y-sl-1), 2*sl + 1) to Direction.S
                    Direction.W -> throw IllegalStateException("unexpected wrap")
                }
            5 ->
                when (dir) {
                    Direction.S ->  // goto 2
                        Position2d(sl - (pos.x-2*sl-1), 2*sl) to Direction.N
                    Direction.N -> throw IllegalStateException("unexpected wrap")
                    Direction.E -> throw IllegalStateException("unexpected wrap")
                    Direction.W ->  // goto 3
                        Position2d(4*sl - (pos.y-3*sl-1), 2*sl) to Direction.N
                }
            6 ->
                when (dir) {
                    Direction.S ->  // goto 2
                        Position2d(1, 1*sl + (pos.x - 3*sl)) to Direction.E
                    Direction.N ->  // goto 4
                        Position2d(3*sl, 2*sl - (pos.x - 3*sl)) to Direction.E
                    Direction.E ->  // goto 1
                        Position2d(3*sl - 1, 3*sl - (pos.y - 2*sl)) to Direction.E
                    Direction.W -> throw IllegalStateException("unexpected wrap")
                }
            else -> throw IllegalStateException("invalid side")
        }
        else
            when(cubeSide(pos)) {
            1 ->
                when (dir) {
                    Direction.S -> throw IllegalStateException("unexpected wrap")
                    Direction.N ->  // goto 5
                        Position2d(1, offsetInverse(pos.x, 3, 1)) to Direction.E
                    Direction.E -> throw IllegalStateException("unexpected wrap")
                    Direction.W ->  // goto 4
                        Position2d(1, 3*sl + 1 - pos.y) to Direction.E
                }
            2 ->
                when (dir) {
                    Direction.S -> throw IllegalStateException("unexpected wrap")
                    Direction.N -> throw IllegalStateException("unexpected wrap")
                    Direction.E ->  // goto 3
                        Position2d(offsetInverse(pos.y, 2, 1), sl) to Direction.N
                    Direction.W ->  // goto 4
                        Position2d(offsetInverse(pos.y, 0, 1), 2*sl+1) to Direction.S
                }
            3 ->
                when (dir) {
                    Direction.S ->  // goto 2
                        Position2d(2*sl, offsetInverse(pos.x, 1, 2)) to Direction.W
                    Direction.N ->  // goto 5
                        Position2d(offsetInverse(pos.x, 0, 2), 4*sl) to Direction.N
                    Direction.E ->  // goto 6
                        Position2d(2*sl, 3*sl + 1 - pos.y) to Direction.W
                    Direction.W -> throw IllegalStateException("unexpected wrap")
                }
            4 ->
                when (dir) {
                    Direction.S -> throw IllegalStateException("unexpected wrap")
                    Direction.N ->  // goto 2
                        Position2d(sl+1, sl + pos.x) to Direction.E
                    Direction.E -> throw IllegalStateException("unexpected wrap")
                    Direction.W ->  // goto 1
                        Position2d(sl+1, 3*sl + 1 - pos.y) to Direction.E
                }
            5 ->
                when (dir) {
                    Direction.S ->  // goto 3
                        Position2d(2*sl + pos.x, 1) to Direction.S
                    Direction.N -> throw IllegalStateException("unexpected wrap")
                    Direction.E ->  // goto 6
                        Position2d(offsetInverse(pos.y, 1, 3), 3*sl) to Direction.N
                    Direction.W ->  // goto 1
                        Position2d(offsetInverse(pos.y, 1, 3), 1) to Direction.S
                }
            6 ->
                when (dir) {
                    Direction.S ->  // goto 5
                        Position2d(sl, offsetInverse(pos.x, 3, 1)) to Direction.W
                    Direction.N -> throw IllegalStateException("unexpected wrap")
                    Direction.E ->  // goto 3
                        Position2d(3*sl, sl + 1 - (pos.y - 2*sl)) to Direction.W
                    Direction.W -> throw IllegalStateException("unexpected wrap")
                }
            else -> throw IllegalStateException("invalid side")
        }
        require(response.first in map.walkable + map.walls)
        if (testMode) {
            require(response.first.x <= 4 * sl)
            require(response.first.y <= 4 * sl)
        } else {
            require(response.first.x <= 3 * sl)
            require(response.first.y <= 4 * sl)
        }
        return response
    }

    fun walkPath() {
        for (step in directions) {
            if (step.turn != null) turn(step.turn)
            else if (step.steps != null) repeat(step.steps) { walkForward() }
            else throw IllegalStateException("invalid PathDirection $step")
        }
    }

    fun prettyPrint(): String = buildString {
        val maxX = map.walkable.maxOf {it.x}
        val maxY = map.walkable.maxOf {it.y}
        for (y in 0 .. maxY) {
            for (x in 0 .. maxX) {
                when {
                    Position2d(x,y) == pos -> this.append(dir.toString())
                    Position2d(x,y) in trace -> this.append('o')
                    Position2d(x,y) in map.walkable -> this.append('.')
                    Position2d(x,y) in map.walls -> this.append('#')
                    else -> this.append(' ')
                }
            }
            this.append('\n')
        }
    }
}

data class PathDirection(val turn: Char?, val steps: Int?) {
    override fun toString() : String = buildString {
        if (turn != null) this.append(turn)
        if (steps != null) this.append(steps)
    }
    companion object {
        fun parseString(path: String) : List<PathDirection> {
            val resultList = mutableListOf<PathDirection>()
            var i = 0
            var numStart = 0
            while (i < path.length) {
                if (path[i].isDigit() && i < path.length-1 && path[i+1].isDigit()) {
                    // longer number
                    i++
                    continue
                }
                else if (path[i].isDigit() && i < path.length-1 && !path[i+1].isDigit()
                    || path[i].isDigit() && i == path.length-1 ) {
                    val num = path.slice(numStart .. i).toInt()
                    resultList += PathDirection(null, num)
                    i++
                }
                else if (path[i].isUpperCase()) {
                    resultList += PathDirection(path[i], null)
                    i++
                    numStart = i
                } else if (path[i].isWhitespace()) {
                    i++
                } else {
                    throw IllegalStateException("unexpected pathDirection input")
                }
            }
            return resultList
        }
    }
}

fun day22(test: Boolean = false) {
    val inputText = if (test)
        """
                ...#
                .#..
                #...
                ....
        ...#.......#
        ........#...
        ..#....#....
        ..........#.
                ...#....
                .....#..
                .#......
                ......#.
        
        10R5L5R10L4R5L5
        """.trimIndent()
      else
        InputTextDownloader().getText(2022, 22)


    val (mapInput, directions) = inputText.split("\n\n")
    val map = MonkeyMap.fromString(mapInput)

    fun password(y: Int, x: Int, d: Int) = (1000 * y) + (4 * x) + d

    // Part A
    val act = Actor(map, PathDirection.parseString(directions))
    act.walkPath()
    println("final position: ${act.pos} facing ${act.dir}")
    println(act.prettyPrint())
    println("password = ${password(act.pos.y, act.pos.x, act.dir.i)}")

    // Part B
    val act2 = Actor(map, PathDirection.parseString(directions), testMode = test)
    act2.partB = true
    act2.walkPath()
    println("final position: ${act2.pos} facing ${act2.dir}")
    println(act2.prettyPrint())
    println("password = ${password(act.pos.y, act.pos.x, act.dir.i)}")
}
