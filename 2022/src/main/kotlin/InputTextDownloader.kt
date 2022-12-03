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
