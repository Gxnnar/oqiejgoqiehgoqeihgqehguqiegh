self.addEventListener("fetch", (event) => {
  let url = new URL(event.request.url);
  let realHost = event.request.referrer.split("/p/")[1];

  if (realHost == undefined) {
    let req = new Request(`/p/${encodeURIComponent(url)}`, event.request);
    return fetch(req);
  }

  realHost = decodeURIComponent(realHost);
  realHost = new URL(realHost).origin;
  let path = url.pathname.split("/p/")[1] ?? url.pathname;

  let req = new Request(
    `/p/${encodeURIComponent(`${realHost}/${path}`)}`,
    event.request
  );
  event.respondWith(fetch(req));
});
