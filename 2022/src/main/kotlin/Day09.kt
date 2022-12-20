import kotlin.math.sign

data class Position2d(var x: Int, var y: Int) {
    infix fun stepTo(other: Position2d): Position2d {
        val newX = x + (other.x - x).sign
        val newY = y + (other.y - y).sign
        return Position2d(newX, newY)
    }

    infix fun adjTo(other: Position2d): Boolean {
        return kotlin.math.abs(x - other.x) <= 1
                && kotlin.math.abs(y - other.y) <= 1
    }
    infix fun notAdjTo(other: Position2d): Boolean = !this.adjTo(other)
    override fun toString() : String = "$x,$y"
    companion object Position2dBuilder {
        fun fromString(input: String) : Position2d {
            val (x, y) = input.split(',').map { it.toInt() }
            return Position2d(x, y)
        }
    }
}

class LongRopeSim : RopeSim() {
    private val multiTailPos = mutableListOf(
        Position2d(0,0),  // head
        Position2d(0,0),
        Position2d(0,0),
        Position2d(0,0),
        Position2d(0,0),
        Position2d(0,0),
        Position2d(0,0),
        Position2d(0,0),
        Position2d(0,0),
        Position2d(0,0),
    )
    override fun toString() : String {
        val (tx, ty) = target
        return buildString {
            append("LongRopeSim(%3d, ".format(step))
            append("target $tx,$ty , rope ")
            append(multiTailPos.mapIndexed { i, pos ->
                val (x, y) = pos
                "$i:$x,$y "
            })
            append(")")
        }
    }

    override fun next(): Boolean {
        if (multiTailPos[0] == target
            && multiTailPos.windowed(2)
                .all { (a,b) -> a adjTo b }
            ) {
            return false  // nothing to do
        }
        step++
        // update from back to front
        for (i in multiTailPos.size-1 downTo 1) {
            val tail = multiTailPos[i]
            val head = multiTailPos[i-1]
            if (tail notAdjTo head) multiTailPos[i] = tail stepTo head
        }
        tailPos = multiTailPos.last()
        tailTrail = tailTrail union setOf(tailPos)

        multiTailPos[0] = multiTailPos[0] stepTo target
        return true
    }
}

open class RopeSim {
    var step = 0
    var tailTrail: Set<Position2d> = mutableSetOf()
    var headPos = Position2d(0,0)
    var tailPos = Position2d(0,0)
    var target = Position2d(0,0)

    init {
        tailTrail = tailTrail union setOf(tailPos)
    }
    override fun toString() : String =
        "RopeSim(%3d, H %2d,%2d, T %2d,%2d)".format(
            step, headPos.x, headPos.y, tailPos.x, tailPos.y)
    fun setTargetTo(dir: Char, dist: Int) {
        target = when (dir) {
            'R' -> Position2d(target.x + dist, target.y)
            'L' -> Position2d(target.x - dist, target.y)
            'U' -> Position2d(target.x, target.y + dist)
            'D' -> Position2d(target.x, target.y - dist)
            else -> { throw Exception("invalid direction") }
        }
    }
    open fun next(): Boolean {
        if (headPos == target && tailPos adjTo headPos) {
            return false  // nothing to do
        }
        step++
        if (tailPos notAdjTo headPos) {
            tailPos = tailPos stepTo headPos
            tailTrail = tailTrail union setOf(tailPos)
        }
        headPos = headPos stepTo target
        return true
    }
}

fun day09(test: Boolean = true) {
    val inputText = if (test)
        """
            R 5
            U 8
            L 8
            D 3
            R 17
            D 10
            L 25
            U 20
        """.trimIndent()
    else
        InputTextDownloader().getText(2022, 9)

    val directions = inputText
        .split('\n')
        .filter { it.isNotEmpty() }
        .map { val (d,i) = it.split(' ')
            d[0] to i.toInt()}
        .also(::println)

    // Part A
    val cur = RopeSim()
    directions.forEach { (d,i) ->
            println("$d $i")
            cur.setTargetTo(d, i)
            while(cur.next()) {
                println(cur)
            }
        }
    println("tail positions: " + cur.tailTrail.size)

    // Part B
    val long = LongRopeSim()
    directions.forEach { (d,i) ->
        println("$d $i")
        long.setTargetTo(d, i)
        while(long.next()) {
            println(long)
        }
    }
    println("long tail positions: " + long.tailTrail.size)
}
