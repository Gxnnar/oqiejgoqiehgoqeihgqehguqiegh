self.addEventListener("fetch", (event) => {
  console.log("====================================");
  console.log("URL: " + event.request.url);
  if (event.request.url.includes("/~/")) return;
  let url = new URL(event.request.url);
  let realHost = event.request.referrer.split("/~/")[1];
  console.log("REFERER: " + event.request.referrer);

  // if (realHost == undefined) {
  //   let req = new Request(`/~/${encodeURIComponent(url)}`, event.request);
  //   return fetch(req);
  // }

  realHost = decodeURIComponent(realHost);
  console.log("REAL HOST" + realHost);
  realHost = new URL(realHost).origin;
  console.log("REAL HOST ORIGIN" + realHost);
  let path = url.pathname + url.search; //.split("/~/")[1] ?? url.pathname;
  console.log("PATH: " + path);

  let req = new Request(
    `/~/${encodeURIComponent(
      `${realHost}${path.endsWith("/") ? "" : "/"}${path}`
    )}`,
    event.request
  );
  console.log("REQ: " + req.url);
  event.respondWith(fetch(req));
});
