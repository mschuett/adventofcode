enum class CrtOp(val cycles: Int) {
    Empty(0),
    Noop(1),
    Addx(2),
}
data class CrtInstruction(val op: CrtOp, val amount: Int)

class CrtSim {
    var cycle = 1
    var instr = CrtInstruction(CrtOp.Empty, 0)
    var remainingOpCycles = instr.op.cycles
    var x = 1
    val telemetry : HashMap<Int,Int> = hashMapOf()

    override fun toString() : String {
        return buildString {
            append("CrtSim(%3d, ".format(cycle))
            append("$remainingOpCycles, ")
            append("$instr, ")
            append("X=$x)")
        }
    }
    fun setOp(newInstr: CrtInstruction) {
        instr = newInstr
        remainingOpCycles = instr.op.cycles
    }
    fun next() : Boolean {
        if (instr.op == CrtOp.Empty)
            return false

        if (cycle % 20 == 0) {
            telemetry[cycle] = x
        }

        remainingOpCycles--
        if (remainingOpCycles == 0) {
            x += instr.amount
            setOp(CrtInstruction(CrtOp.Empty, 0))
        }
        outputBeam()
        cycle++
        return true
    }

    // part B
    var output = ""
    private fun outputBeam() {
        val pixel = (cycle % 40)
        output += if ((x-1) <= pixel && pixel <= (x+1)) {
            '#'
        } else {
            '.'
        }
        if (pixel == 0)
            output += '\n'

    }
}

fun day10(test: Boolean = true) {
    val inputText = if (test)
        """
            addx 15
            addx -11
            addx 6
            addx -3
            addx 5
            addx -1
            addx -8
            addx 13
            addx 4
            noop
            addx -1
            addx 5
            addx -1
            addx 5
            addx -1
            addx 5
            addx -1
            addx 5
            addx -1
            addx -35
            addx 1
            addx 24
            addx -19
            addx 1
            addx 16
            addx -11
            noop
            noop
            addx 21
            addx -15
            noop
            noop
            addx -3
            addx 9
            addx 1
            addx -3
            addx 8
            addx 1
            addx 5
            noop
            noop
            noop
            noop
            noop
            addx -36
            noop
            addx 1
            addx 7
            noop
            noop
            noop
            addx 2
            addx 6
            noop
            noop
            noop
            noop
            noop
            addx 1
            noop
            noop
            addx 7
            addx 1
            noop
            addx -13
            addx 13
            addx 7
            noop
            addx 1
            addx -33
            noop
            noop
            noop
            addx 2
            noop
            noop
            noop
            addx 8
            noop
            addx -1
            addx 2
            addx 1
            noop
            addx 17
            addx -9
            addx 1
            addx 1
            addx -3
            addx 11
            noop
            noop
            addx 1
            noop
            addx 1
            noop
            noop
            addx -13
            addx -19
            addx 1
            addx 3
            addx 26
            addx -30
            addx 12
            addx -1
            addx 3
            addx 1
            noop
            noop
            noop
            addx -9
            addx 18
            addx 1
            addx 2
            noop
            noop
            addx 9
            noop
            noop
            noop
            addx -1
            addx 2
            addx -37
            addx 1
            addx 3
            noop
            addx 15
            addx -21
            addx 22
            addx -6
            addx 1
            noop
            addx 2
            addx 1
            noop
            addx -10
            noop
            noop
            addx 20
            addx 1
            addx 2
            addx 2
            addx -6
            addx -11
            noop
            noop
            noop
        """.trimIndent()
    else
        InputTextDownloader().getText(2022, 10)


    val instructions = inputText
        .split('\n')
        .filter { it.isNotEmpty() }
        .map {
            val words = it.split(' ')
            CrtInstruction(
                CrtOp.valueOf(words.first()),
                if (words.size == 1) 0 else words.last().toInt())
        }
        .also(::println)

    val crt = CrtSim()
    instructions.forEach {
        crt.setOp(it)
        do println(crt)
        while (crt.next())
    }

    // part A
    println("telemetry: ${crt.telemetry}")
    val signalMeasures = listOf(20, 60, 100, 140, 180, 220)
    val signalSum = signalMeasures
        .map { it * crt.telemetry[it]!! }
        .also(::println).sum()
    println("sum of signal strengths: $signalSum")

    // part B
    println(crt.output)
}
