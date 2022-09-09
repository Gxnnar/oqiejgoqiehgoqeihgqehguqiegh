// Request event listaner
self.addEventListener("fetch", (event) => {
  let req = event.request;
  req = new Request(`/p/${encodeURIComponent(req.url)}`, req);

  console.log("WORKER: Fetching", req);
  return fetch(req);
});
