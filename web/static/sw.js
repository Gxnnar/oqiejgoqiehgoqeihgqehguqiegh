// Request event listaner
self.addEventListener("fetch", (event) => {
  let url = new URL(event.request.url);
  let realHost = event.request.referrer.split("/p/")[1] ?? url;
  realHost = new URL(realHost).origin;

  let req = new Request(`/p/${realHost + url.pathname}`, event.request);
  event.respondWith(fetch(req));
});
