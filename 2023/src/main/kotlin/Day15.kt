
fun String.hash(): Int {
    var cur = 0
    for (c in this) {
        cur += c.code
        cur *= 17
        cur %= 256
    }
    return cur
}

// data model is a mess and duplicates info, but it works...
data class Lens(val label: String, var focalLength: Int, var box: Int = 0) {
    override fun toString(): String =
        "[${label} $focalLength]"

    fun focusPower(posInBox: Int): Int =
        (box+1) * (posInBox+1) * focalLength
}

data class LensBox(val lenses: MutableList<Lens>) {
    fun focusPower(): Int =
        lenses.mapIndexed { index, lens ->
            lens.focusPower(index)
        }.sum()
}

data class LensBoxLibrary(val library: List<LensBox> = (0..255).map { LensBox(mutableListOf()) }, val allLenses: MutableList<Lens> = mutableListOf()) {
    fun getLensByLabel(label: String): Lens? {
        val index = allLenses.indexOfFirst { it.label == label }
        return if (index == -1)
            null
        else
            allLenses[index]
    }
    fun addLens(lens: Lens, boxNum: Int) {
        library[boxNum].lenses.add(lens)
        lens.box = boxNum
        allLenses.add(lens)
    }

    fun removeLens(label: String) {
        val box = library[label.hash()].lenses
        val index = box.indexOfFirst { it.label == label }
        if (index != -1) {
            val lens = box[index]
            box.removeAt(index)
            allLenses.remove(lens)
        }
    }

    fun focusPower(): Int =
        library.sumOf {
            it.focusPower()
        }

    override fun toString(): String =
        library.mapIndexedNotNull { i, box ->
            if (box.lenses.isNotEmpty()) {
                "Box ${i}: ${box.lenses}"
            } else null
        }.joinToString("\n")

}

fun day15(test: Boolean = true) {
    val inputText = if (test)
        """
        rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7
        """.trimIndent()
    else
        InputTextDownloader().getText(2023, 15)

    // Part One
    inputText.trim().split(',').sumOf {
        it.hash()
    }.also {
        println("sum: $it")
    }

    // Part Two
    val lib = LensBoxLibrary()

    inputText.trim().split(',').forEach { statement ->
        if ("=" in statement) {
            val (label, fLenText) = statement.split('=')
            val box = label.hash()
            val flen = fLenText.toInt()
            val lens = lib.getLensByLabel(label)
            if (lens != null) {
                lens.focalLength = flen
            } else {
                val newLens = Lens(label, flen, box)
                lib.addLens(newLens, box)
            }
        } else if ("-" in statement) {
            val (label, _) = statement.split('-')
            lib.removeLens(label)
        } else TODO()
    }
    println(lib)
    println(lib.focusPower())
}
