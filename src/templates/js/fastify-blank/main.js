import Fastify from "fastify";
const fastify = Fastify();

fastify.get("/", async function handler(request, reply) {
  return "Hello, world!";
});

try {
  console.log("Server running at: http://127.0.0.1:8000");
  console.log("Available endpoints:");
  console.log("   GET / - show hello world");
  await fastify.listen({ port: 8000, host: "127.0.0.1" });
} catch (err) {
  fastify.log.error(err);
  process.exit(1);
}
