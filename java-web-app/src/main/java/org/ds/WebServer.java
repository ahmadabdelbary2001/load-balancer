package org.ds;

import com.sun.net.httpserver.HttpExchange;
import com.sun.net.httpserver.HttpServer;
import org.jsoup.Jsoup;
import org.jsoup.nodes.Document;
import org.jsoup.nodes.Element;

import java.io.IOException;
import java.io.InputStream;
import java.io.OutputStream;
import java.net.InetSocketAddress;
import java.util.concurrent.Executors;

// build artifacts then
// run with this java -jar .\out\artifacts\simpleWebApp_jar\simpleWebApp.jar 9090 "server 1"
public class WebServer {
    private static final String STATUS_ENDPOINT = "/status";
    private static final String HOME_PAGE_ENDPOINT = "/";
    private static final String HOLD_ENDPOINT = "/hold";

    private static final String HTML_PAGE = "/index.html";

    private final int port;
    private HttpServer server;
    private final String serverName;

    public WebServer(int port, String serverName) {
        this.port = port;
        this.serverName = serverName;
    }

    public void startServer() {
        try {
            this.server = HttpServer.create(new InetSocketAddress(port), 0);
        } catch (IOException e) {
            e.printStackTrace();
            return;
        }

        server.createContext(STATUS_ENDPOINT, this::handleStatusCheckRequest);
        server.createContext(HOME_PAGE_ENDPOINT, this::handleHomePageRequest);
        server.createContext(HOLD_ENDPOINT, this::handleHoldRequest);

        server.setExecutor(Executors.newFixedThreadPool(8));
        System.out.println(String.format("Started server %s on port %d ", serverName, port));
        server.start();
    }

    private void handleHomePageRequest(HttpExchange exchange) throws IOException {
        if (!exchange.getRequestMethod().equalsIgnoreCase("get")) {
            exchange.close();
            return;
        }

        System.out.println(String.format("%s received a request", this.serverName));
        exchange.getResponseHeaders().add("Content-Type", "text/html");
        exchange.getResponseHeaders().add("Cache-Control", "no-cache");

        byte[] response = loadHtml(HTML_PAGE);

        sendResponse(response, exchange);
    }

    /**
     * Loads the HTML page to be fetched to the web browser
     *
     * @param htmlFilePath - The relative path to the html file
     * @throws IOException
     */
    private byte[] loadHtml(String htmlFilePath) throws IOException {
        InputStream htmlInputStream = getClass().getResourceAsStream(htmlFilePath);
        if (htmlInputStream == null) {
            return new byte[]{};
        }

        Document document = Jsoup.parse(htmlInputStream, "UTF-8", "");

        String modifiedHtml = modifyHtmlDocument(document);
        return modifiedHtml.getBytes();
    }

    /**
     * Fills the server's name and local time in theHTML document
     *
     * @param document - original HTML document
     */
    private String modifyHtmlDocument(Document document) {
        Element serverNameElement = document.selectFirst(".server-name"); // Use class selector matching index.html
        if (serverNameElement != null) {
            serverNameElement.text(serverName);
        }
        return document.toString();
    }

    private void handleHoldRequest(HttpExchange exchange) throws IOException {
        String query = exchange.getRequestURI().getQuery();
        int duration = 0;
        if (query != null && query.contains("duration=")) {
            try {
                duration = Integer.parseInt(query.split("=")[1]);
            } catch (NumberFormatException e) {
                duration = 0;
            }
        }

        System.out.println(String.format("%s: Holding connection for %d seconds...", serverName, duration));
        
        try {
            if (duration > 0) {
                Thread.sleep(duration * 1000L);
            }
        } catch (InterruptedException e) {
            e.printStackTrace();
        }

        String response = "Connection released after " + duration + "s";
        sendResponse(response.getBytes(), exchange);
        System.out.println(String.format("%s: Connection released.", serverName));
    }

    private void handleStatusCheckRequest(HttpExchange exchange) throws IOException {
        if (!exchange.getRequestMethod().equalsIgnoreCase("get")) {
            exchange.close();
            return;
        }

        System.out.println("Received a health check");
        String responseMessage = "Server is alive\n";
        sendResponse(responseMessage.getBytes(), exchange);
    }

    private void sendResponse(byte[] responseBytes, HttpExchange exchange) throws IOException {
        exchange.sendResponseHeaders(200, responseBytes.length);
        OutputStream outputStream = exchange.getResponseBody();
        outputStream.write(responseBytes);
        outputStream.flush();
        outputStream.close();
    }
}