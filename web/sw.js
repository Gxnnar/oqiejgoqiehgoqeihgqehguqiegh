const HOSTNAMES = ["http://localhost:8080/"];
const ABSOLUTE_URL_REGEX = new RegExp("^(?:[a-z+]+:)?//", "i");

self.addEventListener("fetch", (event) => {
  if (HOSTNAMES.includes(event.request.referrer)) return;

  let url = new URL(event.request.url);
  let realHost = event.request.referrer.split("/p/")[1] ?? url;
  realHost = decodeURIComponent(realHost);
  realHost = new URL(realHost).origin;

  let req = new Request(
    `/p/${encodeURIComponent(realHost + url.pathname)}`,
    event.request
  );
  event.respondWith(fetch(req));
});
