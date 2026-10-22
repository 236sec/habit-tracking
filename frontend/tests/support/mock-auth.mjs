import http from "node:http";

function reply(response, status, body) {
  response.writeHead(status, { "content-type": "application/json" });
  response.end(JSON.stringify(body));
}

for (const [provider, port] of [
  ["google", 4101],
  ["github", 4102],
]) {
  const user = {
    id: `${provider}-test-subject`,
    aud: "authenticated",
    role: "authenticated",
    email: "same@example.test",
    app_metadata: { provider },
    user_metadata: {},
    created_at: "2026-01-01T00:00:00Z",
  };
  http
    .createServer((request, response) => {
      const url = new URL(request.url, `http://127.0.0.1:${port}`);
      if (url.pathname === "/health") return reply(response, 200, { ok: true });
      if (url.pathname === "/auth/v1/authorize") {
        const destination = new URL(url.searchParams.get("redirect_to"));
        destination.searchParams.set("code", `${provider}-test-code`);
        response.writeHead(302, { location: destination.toString() });
        return response.end();
      }
      if (url.pathname === "/auth/v1/token") {
        return reply(response, 200, {
          access_token: `${provider}-test-token`,
          refresh_token: `${provider}-refresh-token`,
          token_type: "bearer",
          expires_in: 3600,
          user,
        });
      }
      if (url.pathname === "/auth/v1/user") return reply(response, 200, user);
      if (url.pathname === "/auth/v1/logout") return reply(response, 204, {});
      return reply(response, 404, { error: "Unknown mock auth route" });
    })
    .listen(port, "127.0.0.1");
}

http
  .createServer((request, response) => {
    if (request.url === "/health") return reply(response, 200, { ok: true });
    if (request.url === "/api/me") {
      const token = request.headers.authorization?.replace(/^Bearer /, "");
      if (token === "google-test-token")
        return reply(response, 200, {
          success: true,
          data: { id: "11111111-1111-4111-8111-111111111111" },
        });
      if (token === "github-test-token")
        return reply(response, 200, {
          success: true,
          data: { id: "22222222-2222-4222-8222-222222222222" },
        });
      return reply(response, 401, { success: false });
    }
    return reply(response, 404, {});
  })
  .listen(4103, "127.0.0.1");
