import kotlin.math.sign

enum class SnaFuDigit(val i: Int, val c: Char) {
    Two(2, '2'),
    One(1, '1'),
    Zero(0, '0'),
    Minus(-1, '-'),
    DoubleMinus(-2, '=');
    companion object {
        fun fromChar(c: Char): SnaFuDigit =
            SnaFuDigit.values().find { c == it.c }!!

        fun fromInt(i: Int): SnaFuDigit {
            require(-2 <= i && i <= 2)
            return SnaFuDigit.values().find { i == it.i }!!
        }
    }
}

data class SnaFuNum(var num: Long) {
    override fun toString() : String {
        var out = charArrayOf()
        var i = num+2*num.sign

        do {
            val rem = i.rem(5)
            val div = i.div(5)
            val digit = SnaFuDigit.fromInt((rem-(2*i.sign)).toInt())
            out += digit.c
            i = i.div(5)+(2*i.sign)
        } while (div != 0L)
        return out.reversed().joinToString("")
    }
    companion object {
        fun fromString(input: String): SnaFuNum {
            var num = 0L
            var place = 1L
            for (digit in input.reversed()) {
                val sd = SnaFuDigit.fromChar(digit)
                val placenum: Long = place * sd.i
                num += placenum
                place *= 5
            }
            return SnaFuNum(num)
        }
    }
}


fun day25(test: Boolean = false) {
    val inputText = if (test)
        """
        1=-0-2
        12111
        2=0=
        21
        2=01
        111
        20012
        112
        1=-1=
        1-12
        12
        1=
        122
        """.trimIndent()
    else
        InputTextDownloader().getText(2022, 25)

    inputText
        .split('\n')
        .filter { it.isNotEmpty() }
        .map {
            SnaFuNum.fromString(it)
        }
        .sumOf { it.num }
        .also { println("sum is $it (decimal) or ${SnaFuNum(it)} (snafu)") }
}
