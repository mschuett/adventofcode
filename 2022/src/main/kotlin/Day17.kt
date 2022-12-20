import kotlin.math.pow

enum class TetrisRock(val i: Int, val bitmap: Array<Array<Boolean>>) {
    A(0, bitmap = arrayOf(
        arrayOf(true, true, true, true))
    ),
    B(1, bitmap = arrayOf(
        arrayOf(false, true, false),
        arrayOf(true,  true, true ),
        arrayOf(false, true, false),
    )),
    C(2, bitmap = arrayOf(
        arrayOf(false, false, true),
        arrayOf(false, false, true),
        arrayOf( true,  true, true),
    )),
    D(3, bitmap = arrayOf(
        arrayOf(true),
        arrayOf(true),
        arrayOf(true),
        arrayOf(true),
    )),
    E(4, bitmap = arrayOf(
        arrayOf(true, true),
        arrayOf(true, true),
    ))
}

data class TetrisSprite(val rock: TetrisRock, var pos: Position2d) {
    fun nextRock() : TetrisRock {
        return TetrisRock.valueOf(('A' + ((rock.i + 1) % 5)).toString())
    }
    fun stepDown() { pos.y-- }
    fun revertStepDown() { pos.y++ }
    fun minX() = pos.x
    fun maxX() = pos.x + rock.bitmap[0].size - 1

    fun applyJet(jet: Char, caveWidth: Int) {
        when(jet) {
            '<' -> if (minX() > 1) pos.x--
            '>' -> if (maxX() < caveWidth) pos.x++
        }
    }
    fun revertApplyJet(jet: Char) {
        when(jet) {
            '<' -> pos.x++
            '>' -> pos.x--
        }
    }
    fun genAllPositions() = sequence<Position2d> {
        for (y in 0 until rock.bitmap.size)
            for (x in 0 until rock.bitmap[0].size)
                if (rock.bitmap[rock.bitmap.size-1-y][x])
                    yield(Position2d(pos.x+x,pos.y+y))

    }
    operator fun contains(queryPos: Position2d): Boolean {
        // rock position is always the lower left pixel/coordinate of the sprite
        // -> check if in bitmap bounding box, then check bitmap
        val offsetX = queryPos.x - pos.x
        val offsetY = queryPos.y - pos.y
        return ((offsetX >= 0 && offsetX < rock.bitmap[0].size)
                && (offsetY >= 0 && offsetY < rock.bitmap.size)
                && rock.bitmap[rock.bitmap.size-1-offsetY][offsetX]
            )
    }
    infix fun collidesWith(map: MutableSet<Position2d>) : Boolean{
        for (y in 0 until rock.bitmap.size)
            for (x in 0 until rock.bitmap[0].size)
                if (rock.bitmap[rock.bitmap.size-1-y][x] && Position2d(pos.x+x,pos.y+y) in map)
                    return true
        return false
    }
    infix fun collidesWith(map: ArrayList<ByteArray>) : Boolean {
        for (y in 0 until rock.bitmap.size)
            for (x in 0 until rock.bitmap[0].size)
                if (rock.bitmap[rock.bitmap.size-1-y][x] && map.contains(Position2d(pos.x+x,pos.y+y)))
                    return true
        return false
    }
}

infix fun ArrayList<ByteArray>.contains(pos: Position2d) : Boolean {
    return this.size > pos.y && this[pos.y][pos.x] != 0.toByte()
}
fun ArrayList<ByteArray>.add(pos: Position2d) {
    while(this.size <= pos.y)
        this.add(ByteArray(8) { 0 })
    this[pos.y][pos.x] = 1
}
fun ArrayList<ByteArray>.addAll(coll: Collection<Position2d>) {
    coll.forEach {
        this.add(it)
    }
}

class TetrisCave(val jets: CharArray) {
    val caveWidth: Int = 7
    val mapArray : ArrayList<ByteArray> = arrayListOf()
    var curObj : TetrisSprite = TetrisSprite(TetrisRock.A, origin())
    var lastObj : TetrisSprite = TetrisSprite(TetrisRock.A, origin())  // only for prettyPrinting
    var stepCount = 0L
    var rockCount = 1L
    var maxY : Int = 0
    private fun origin() : Position2d = Position2d(3, maxY + 4)

    private fun genNewRock() {
        lastObj = curObj
        curObj = TetrisSprite(curObj.nextRock(), origin())
        rockCount++
    }

    private fun persistObj() {
        val allPositions = curObj.genAllPositions().toList()
        allPositions.forEach { require(!mapArray.contains(it)) }
        mapArray.addAll(allPositions)
        maxY = maxOf(maxY, allPositions.maxBy { it.y }.y)
    }
    fun nextStep(interactive: Boolean = false) {
        require(mapArray.isEmpty() || mapArray.size == maxY+1)
        val jet: Char = jets[(stepCount % jets.size).toInt()]

        if (interactive) {  // for debugging
            println("\u001Bc step: $stepCount, rocks: $rockCount, jet: $jet")
            println(prettyPrint())
            readln()
        }
        stepCount++

        // try the applyJet, then do check, revert if necessary
        curObj.applyJet(jet, caveWidth)
        if (curObj collidesWith mapArray)
            curObj.revertApplyJet(jet)

        // try the stepDown, then do check, revert if necessary
        curObj.stepDown()
        if (curObj.pos.y <= maxY && !(!(curObj collidesWith mapArray) && curObj.pos.y > 0)) {
            // collision -> revert and persist old position
            curObj.revertStepDown()
            persistObj()
            genNewRock()
            return
        }
    }
    fun prettyPrint() = buildString {
        val red = "\u001b[31m"
        val reset = "\u001b[0m"
        for (y in maxY+8 downTo 0) {
            for (x in 0..caveWidth+1) {
                if (y == 0 ) {
                    this.append('-')
                    continue
                }
                if (x == 0 || x == caveWidth+1) {
                    this.append('|')
                    continue
                }
                val pos = Position2d(x, y)
                if (pos in curObj) {
                    this.append('@')
                    continue
                }
                if (pos in lastObj) {
                    this.append(red, '#', reset)
                    continue
                }
                if (mapArray.contains(pos)) {
                    this.append('#')
                    continue
                }
                this.append('.')
            }
            this.append('\n')
        }
    }

    // Part B
    val oldStates : MutableMap<TetrisStackState, Pair<Long,Int>> = mutableMapOf()
    fun checkLoop() : Pair<Long,Int> {
        if (maxY < 100) return 0L to 0

        val mapTop = (1..10)
            .map { mapArray[maxY - it] }
            .map {
                // save lines as Integers to make them smaller and comparable
                it.mapIndexed { i, byte ->
                    byte * (2.toDouble().pow(i)).toInt()
                }.sum()
            }

        val curState = TetrisStackState(
            curObj.rock,
            (stepCount % jets.size).toInt(),
            mapTop
        )

        if (curState in oldStates) {
            val (oldRockCount, oldMaxY) = oldStates[curState]!!
            val loopCycle = rockCount - oldRockCount
            val yCycle = maxY - oldMaxY
            // println("found loop of length $loopCycle with Y diff $yCycle")
            return loopCycle to yCycle
        } else {
            oldStates[curState] = rockCount to maxY
            return 0L to 0
        }
    }
}

data class TetrisStackState(val curRock: TetrisRock, val curJetPos: Int, val curMapTop: List<Int>)

fun day17(test: Boolean = true) {
    val inputText = if (test)
        """>>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>""".trimIndent()
    else
        InputTextDownloader().getText(2022, 17)

    val jets = inputText
        .trim()
        .toCharArray()

    val cave = TetrisCave(jets)

    // part A
    while (cave.rockCount <= 2022) {
        cave.nextStep()
    }
    println("rock #${cave.rockCount} created\ntower height is ${cave.maxY}")
    //println(cave.prettyPrint())

    // part B
    val trillion = 1_000_000_000_000L
    var loopLen = 0L
    var yCycle = 0
    while (loopLen == 0L && cave.rockCount <= trillion) {
        cave.nextStep()
        val checkLoop = cave.checkLoop()
        loopLen = checkLoop.first
        yCycle  = checkLoop.second
    }
    println("found loopLen $loopLen with yDiff $yCycle at rock #${cave.rockCount} and height ${cave.maxY}")
    val cycles = (trillion - cave.rockCount).div(loopLen)
    val skippingYGrowth = cycles * yCycle
    val skippingRocks = cycles * loopLen
    println("skipping cycles $cycles, rocks $skippingRocks, and Y growth $skippingYGrowth")
    // do the missing rocks now
    while (cave.rockCount + skippingRocks <= trillion) {
        cave.nextStep()
    }
    println("rock #${cave.rockCount + skippingRocks} created\ntower height is ${cave.maxY + skippingYGrowth}")
}
