import java.net.URI
import java.net.http.HttpClient
import java.net.http.HttpRequest
import java.net.http.HttpResponse

/* helper function to download input texts */
class InputTextDownloader {
    private val sessionId = System.getenv("ADVENT_AUTH_SESSION_ID")
    private val client = HttpClient.newBuilder()
        .followRedirects(HttpClient.Redirect.ALWAYS)
        .build()

    fun getText(year: Int, day: Int) : String {
        val request = HttpRequest.newBuilder()
            .uri(URI.create("https://adventofcode.com/${year}/day/${day}/input"))
            .header("Cookie", "session=$sessionId")
            .build()
        val response = client.send(request, HttpResponse.BodyHandlers.ofString())
        return response.body()
    }
}

fun day1() {
    val test = false

    val inputText = if (test)
        """
            1000
            2000
            3000
            
            4000
            
            5000
            6000
            
            7000
            8000
            9000
            
            10000
        """.trimIndent()
    else
        InputTextDownloader().getText(2022, 1)

    val elves = ArrayList<ArrayList<Int>>(30)

    var elf = ArrayList<Int>(5)
    for (line in inputText.split('\n')) {
        if (line.isNotEmpty()) {
            elf.add(line.toInt())
        } else {
            elves.add(elf)
            elf = ArrayList(5)
        }
    }
    elves.add(elf)
    println(elves)

    // Part a
    var maxCalories = 0
    for (carry in elves) {
        val calories = carry.sum()
        if (calories > maxCalories) {
            maxCalories = calories
        }
    }
    println("max calories are $maxCalories")

    // Part b
    val sums = ArrayList(elves.map { arr -> arr.sum() })
    sums.sortDescending()

    println("sum: $sums")
    val topThree = sums.slice(0..2)
    println("top three elves are: $topThree")
    println("sum: " + topThree.sum())
}

fun main() {
    day1()
}
