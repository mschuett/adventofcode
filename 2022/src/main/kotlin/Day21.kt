import java.util.*

enum class MonkeyOp(val c: Char, val f: (Long, Long) -> Long) {
    PLUS ('+', { x: Long, y: Long -> x + y }),
    MINUS('-', { x: Long, y: Long -> x - y }),
    MULT ('*', { x: Long, y: Long -> x * y }),
    DIV  ('/', { x: Long, y: Long -> x / y });
    fun inverse() : MonkeyOp = when (this) {
        PLUS -> MINUS
        MINUS -> PLUS
        MULT -> DIV
        DIV -> MULT
    }
    companion object {
        fun fromChar(c: Char): MonkeyOp {
            return MonkeyOp.values().find { it.c == c}!!
        }
    }
}

abstract class MonkeyNode(val id: String) {
    var parent: MonkeyOpNode? = null
    abstract fun eval() : Long
}

class MonkeyNumberNode(id: String, var num: Long) : MonkeyNode(id) {
    override fun eval(): Long = num
}
class MonkeyOpNode(id: String, val op: MonkeyOp, val left: MonkeyNode, val right: MonkeyNode) : MonkeyNode(id) {
    init {
        left.parent = this
        right.parent = this
    }
    override fun eval(): Long = op.f(left.eval(), right.eval())
}

// Map is not really required, but it is useful to lookup by id
fun parseMonkeyTrees(input: String): Map<String, MonkeyNode> {
    val names : MutableMap<String, MonkeyNode> = mutableMapOf()

    val lines : Queue<String> = ArrayDeque(
        input
        .split('\n')
        .filter { it.isNotEmpty() }
    )

    while (lines.isNotEmpty()) {
        val line = lines.remove()
        val (id,rest) = line.split(": ")
        if (rest.length < 8) {  // number = leaf node
            names[id] = MonkeyNumberNode(id, rest.toLong())
        } else {
            val (left, op, right) = rest.split(' ')
            if (left !in names || right !in names) {
                // put line back to queue until we have the child nodes
                lines.add(line)
            } else {
                // process for real
                names[id] = MonkeyOpNode(id, MonkeyOp.fromChar(op[0]), names[left]!!, names[right]!!)
            }
        }
    }
    return names
}

fun solveToUnknownInput(monkeys: Map<String, MonkeyNode>): Long {
    fun determineOps(curMonkey: MonkeyOpNode, lastMonkey: MonkeyNode): List<Triple<Long?, MonkeyOp?, Long?>> {
        if (lastMonkey == curMonkey.left) {
            val curNumber = curMonkey.right.eval()
            if (curMonkey.parent == null) {  // root node
                return listOf(Triple(null, null, curNumber))
            }
            return determineOps(curMonkey.parent!!, curMonkey) + Triple(null, curMonkey.op, curNumber)
        } else {
            val curNumber = curMonkey.left.eval()
            if (curMonkey.parent == null) {  // root node
                return listOf(Triple(curNumber, null, null))
            }
            return determineOps(curMonkey.parent!!, curMonkey) + Triple(curNumber, curMonkey.op, null)
        }
    }

    fun reverseOps(result: Long, curOp: Triple<Long?, MonkeyOp?, Long?>): Long {
        val (a, op, b) = curOp
        require(op != null)
        // null is our X, the other is the known given
        return if (a != null) {
            when (op) {
                MonkeyOp.PLUS, MonkeyOp.MULT -> op.inverse().f(result, a)
                MonkeyOp.MINUS, MonkeyOp.DIV -> op.f(a, result)  // non-symmetry here
            }
        } else if (b != null) {
            when (op) {
                MonkeyOp.PLUS, MonkeyOp.MULT -> op.inverse().f(result, b)
                MonkeyOp.MINUS, MonkeyOp.DIV -> op.inverse().f(b, result)
            }
        } else {
            throw IllegalStateException("impossible")
        }
    }

    val recList = determineOps(monkeys["humn"]!!.parent!!, monkeys["humn"]!!)
    var result = recList.first().first ?: recList.first().third
    require(result != null)
    for (opItem in recList.drop(1)) {
        require(opItem.second != null)
        result = reverseOps(result!!, opItem)
        // println("result = $result, from $opItem")
    }
    return result!!
}

fun day21(test: Boolean = true) {
    val inputText = if (test)
        """
        root: pppw + sjmn
        dbpl: 5
        cczh: sllz + lgvd
        zczc: 2
        ptdq: humn - dvpt
        dvpt: 3
        lfqf: 4
        humn: 5
        ljgn: 2
        sjmn: drzm * dbpl
        sllz: 4
        pppw: cczh / lfqf
        lgvd: ljgn * ptdq
        drzm: hmdt - zczc
        hmdt: 32
        """.trimIndent()
      else
        InputTextDownloader().getText(2022, 21)

    // Part A
    val monkeys = parseMonkeyTrees(inputText)
    println("root monkey result: " + monkeys["root"]!!.eval())

    // Part B
    val result = solveToUnknownInput(monkeys)
    // counter check
    (monkeys["humn"]!! as MonkeyNumberNode).num = result
    val root = monkeys["root"]!! as MonkeyOpNode
    val rootLeft = root.left.eval()
    val rootRight = root.right.eval()
    if (rootLeft == rootRight) {
        println("$rootLeft == $rootRight")
        println("humn number is $result")
    } else {
        println("$rootLeft != $rootRight")
        println("humn number is NOT $result")
    }
}
