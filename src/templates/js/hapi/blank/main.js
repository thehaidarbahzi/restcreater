import { server as _server } from "@hapi/hapi";

const init = async () => {
  const server = _server({
    port: 8000,
    host: "127.0.0.1",
  });

  server.route({
    method: "GET",
    path: "/",
    handler: (request, h) => {
      return "Hello, world!";
    },
  });

  await server.start();
  console.log("Server running on %s", server.info.uri);
  console.log("Available endpoints:");
  console.log("   GET / - show hello world");
};

process.on("unhandledRejection", (err) => {
  console.log(err);
  process.exit(1);
});

init();
