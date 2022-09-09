// Request event listaner
self.addEventListener("fetch", (event) => {
  console.log("HANDLING", event);

  let req = event.request;
  let url = new URL(req.url);
  let realHost = req.headers.get("Referer").split("/p/")[1];

  if (url.hostname == "localhost" && !url.pathname.startsWith("/p/"))
    req = new Request(`/p/${encodeURIComponent(realHost + url.pathname)}`, req);

  console.log("WORKER: Fetching", req);
  return fetch(req).then((d) => {
    return d;
  });
});
